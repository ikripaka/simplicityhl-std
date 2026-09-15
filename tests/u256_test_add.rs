mod common;

use primitive_types::U256;

use crate::common::helper::generate_u256;
use common::core::{Expect, run};

use simplicityhl_std::artifacts::u256_test_add::U256TestAddProgram;
use simplicityhl_std::artifacts::u256_test_add::derived_u256_test_add::{
    U256TestAddArguments, U256TestAddWitness,
};

enum FunctionToTest {
    Add256,
    Add256_128,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

fn program() -> U256TestAddProgram {
    U256TestAddProgram::new(U256TestAddArguments {})
}

fn build_witness(
    function: u8,
    a: [u8; 32],
    b: [u8; 32],
    expected: Option<[u8; 32]>,
    expected_bool: bool,
) -> U256TestAddWitness {
    U256TestAddWitness {
        function_index: function,
        first_arg: a,
        second_arg: b,
        expected,
        expected_bool,
    }
}

mod u256_tests_arithmetic {
    use super::*;

    #[simplex::test]
    fn u256_test_add_256_not_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX / 2);
        let b = generate_u256(U256::zero(), U256::MAX / 2);
        let result = (a + b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Add256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result),
                false,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_add_256_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = U256::MAX;
        let b = generate_u256(U256::one(), U256::MAX);
        let result = (b - 1).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Add256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result),
                true,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_add_256_128_not_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX / 2);
        let b = generate_u256(U256::one(), U256::from(u128::MAX));
        let result = (a + b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Add256_128),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result),
                false,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_add_256_128_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = U256::MAX;
        let b = generate_u256(U256::one(), U256::from(u128::MAX));
        let result = (b - 1).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Add256_128),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result),
                true,
            ),
            Expect::Ok,
        )
    }
}

mod u256_tests_arithmetic_fuzz {
    use super::*;

    use common::core::{Expect, FuzzExecutionCheck};
    use simplex::fuzz;
    use simplex::fuzz::builders::{FinalTransactionBuilder, ProgramTarget};
    use simplex::fuzz::engine::FuzzStrategyBuilder;
    use simplex::fuzz::proptest::prelude::{Just, any};
    use simplex::fuzz::proptest::strategy::{BoxedStrategy, Strategy};
    use simplex::fuzz::{FuzzEngineBuilder, FuzzError};
    use simplex::simplicityhl::{Arguments, WitnessValues};
    use simplex::transaction::{FinalTransaction, PartialInput, RequiredSignature, UTXO};

    const PROGRAM_TARGET: ProgramTarget = ProgramTarget::Input(0);
    const EXPECTED_TRUE: bool = true;
    const EXPECTED_FALSE: bool = false;

    type U256AddFuzzEngineBuilder =
        FuzzEngineBuilder<U256TestAddProgram, U256TestAddArguments, U256TestAddWitness>;

    fn initial_transaction() -> FinalTransaction {
        let mut tx = FinalTransaction::new();
        tx.add_input(PartialInput::new(UTXO::default()), RequiredSignature::None);
        tx
    }

    fn transaction_builder() -> Result<FinalTransactionBuilder, FuzzError> {
        FinalTransactionBuilder::new(initial_transaction(), [PROGRAM_TARGET])
    }

    fn arb_u256_be() -> impl Strategy<Value = U256> {
        any::<[u8; 32]>().prop_map(|bytes| U256::from_big_endian(&bytes))
    }

    fn arb_non_zero_u256_be() -> impl Strategy<Value = U256> {
        arb_u256_be().prop_filter("u256 should not be zero", |value| !value.is_zero())
    }

    fn arb_u128() -> impl Strategy<Value = u128> {
        any::<u128>()
    }

    fn arb_non_zero_u128() -> impl Strategy<Value = u128> {
        arb_u128().prop_filter("u128 should not be zero", |value| *value != 0)
    }

    fn arb_128bit_u256_be() -> impl Strategy<Value = U256> {
        arb_u128().prop_map(U256::from)
    }

    fn arb_non_zero_128bit_u256_be() -> impl Strategy<Value = U256> {
        arb_non_zero_u128().prop_map(U256::from)
    }

    fn arb_255bit_u256_be() -> impl Strategy<Value = U256> {
        arb_u256_be().prop_map(|x| x >> 1)
    }

    fn addition_strategy(
        function_index: u8,
        expected_bool: bool,
        inputs: BoxedStrategy<(U256, U256)>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U256TestAddArguments, U256TestAddWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(a, b)| {
                let (expected, _) = a.overflowing_add(b);
                let arguments: Arguments = U256TestAddArguments {}.into();
                let witness: WitnessValues = build_witness(
                    function_index,
                    a.to_big_endian(),
                    b.to_big_endian(),
                    Some(expected.to_big_endian()),
                    expected_bool,
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_successful_fuzz(
        fuzz_engine_builder: U256AddFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u256 addition", Expect::Ok));

        Ok(())
    }

    #[simplex::fuzz]
    fn u256_test_add_256_not_overflow(
        fuzz_engine_builder: U256AddFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            addition_strategy(
                op(FunctionToTest::Add256),
                EXPECTED_FALSE,
                (arb_255bit_u256_be(), arb_255bit_u256_be()).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_add_256_overflow(
        fuzz_engine_builder: U256AddFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            addition_strategy(
                op(FunctionToTest::Add256),
                EXPECTED_TRUE,
                (Just(U256::MAX), arb_non_zero_u256_be()).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_add_256_128_not_overflow(
        fuzz_engine_builder: U256AddFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            addition_strategy(
                op(FunctionToTest::Add256_128),
                EXPECTED_FALSE,
                (arb_255bit_u256_be(), arb_128bit_u256_be()).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_add_256_128_overflow(
        fuzz_engine_builder: U256AddFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            addition_strategy(
                op(FunctionToTest::Add256_128),
                EXPECTED_TRUE,
                (Just(U256::MAX), arb_non_zero_128bit_u256_be()).boxed(),
            ),
        )
    }
}
