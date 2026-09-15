mod common;

use primitive_types::U256;

use crate::common::helper::generate_u256;
use common::core::{Expect, run};

use simplicityhl_std::artifacts::u256_test_compare::U256TestCompareProgram;
use simplicityhl_std::artifacts::u256_test_compare::derived_u256_test_compare::{
    U256TestCompareArguments, U256TestCompareWitness,
};

enum FunctionToTest {
    IsZero256,
    Lt256,
    Le256,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

const DEFAULT_EXPECTED: [u8; 32] = [0; 32];

fn program() -> U256TestCompareProgram {
    U256TestCompareProgram::new(U256TestCompareArguments {})
}

fn build_witness(
    function: u8,
    a: [u8; 32],
    b: [u8; 32],
    expected_bool: bool,
) -> U256TestCompareWitness {
    U256TestCompareWitness {
        function_index: function,
        first_arg: a,
        second_arg: b,
        expected_bool,
    }
}

mod u256_tests_compare {
    use super::*;

    #[simplex::test]
    fn u256_test_is_zero_256_true(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = [0; 32];

        run(
            &context,
            program(),
            build_witness(op(FunctionToTest::IsZero256), a, DEFAULT_EXPECTED, true),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_is_zero_256_false(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::one(), U256::MAX).to_big_endian();

        run(
            &context,
            program(),
            build_witness(op(FunctionToTest::IsZero256), a, DEFAULT_EXPECTED, false),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_lt_256_less(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX - 1);
        let b = a + 1;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Lt256),
                a.to_big_endian(),
                b.to_big_endian(),
                true,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_lt_256_eq(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX).to_big_endian();

        run(
            &context,
            program(),
            build_witness(op(FunctionToTest::Lt256), a, a, false),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_lt_256_bigger(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::one(), U256::MAX);
        let b = a - 1;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Lt256),
                a.to_big_endian(),
                b.to_big_endian(),
                false,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_le_256_less(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX - 1);
        let b = a + 1;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Le256),
                a.to_big_endian(),
                b.to_big_endian(),
                true,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_le_256_eq(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::one(), U256::MAX).to_big_endian();

        run(
            &context,
            program(),
            build_witness(op(FunctionToTest::Le256), a, a, true),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_le_256_bigger(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::one(), U256::MAX);
        let b = a - 1;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Le256),
                a.to_big_endian(),
                b.to_big_endian(),
                false,
            ),
            Expect::Ok,
        )
    }
}

mod u256_tests_compare_fuzz {
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

    type U256CompareFuzzEngineBuilder =
        FuzzEngineBuilder<U256TestCompareProgram, U256TestCompareArguments, U256TestCompareWitness>;

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

    fn arb_255bit_u256_be() -> impl Strategy<Value = U256> {
        arb_u256_be().prop_map(|value| value >> 1)
    }

    fn comparison_strategy(
        function_index: u8,
        expected_bool: bool,
        inputs: BoxedStrategy<(U256, U256)>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U256TestCompareArguments, U256TestCompareWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(a, b)| {
                let arguments: Arguments = U256TestCompareArguments {}.into();
                let witness: WitnessValues = build_witness(
                    function_index,
                    a.to_big_endian(),
                    b.to_big_endian(),
                    expected_bool,
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_successful_fuzz(
        fuzz_engine_builder: U256CompareFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u256 comparison", Expect::Ok));

        Ok(())
    }

    #[simplex::fuzz]
    fn u256_test_compare_is_zero_256_true(
        fuzz_engine_builder: U256CompareFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            comparison_strategy(
                op(FunctionToTest::IsZero256),
                EXPECTED_TRUE,
                Just((U256::zero(), U256::zero())).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_compare_is_zero_256_false(
        fuzz_engine_builder: U256CompareFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            comparison_strategy(
                op(FunctionToTest::IsZero256),
                EXPECTED_FALSE,
                arb_non_zero_u256_be()
                    .prop_map(|a| (a, U256::zero()))
                    .boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_compare_lt_256_less(
        fuzz_engine_builder: U256CompareFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            comparison_strategy(
                op(FunctionToTest::Lt256),
                EXPECTED_TRUE,
                arb_255bit_u256_be()
                    .prop_map(|a| (a, a + U256::one()))
                    .boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_compare_lt_256_eq(
        fuzz_engine_builder: U256CompareFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            comparison_strategy(
                op(FunctionToTest::Lt256),
                EXPECTED_FALSE,
                arb_u256_be().prop_map(|value| (value, value)).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_compare_lt_256_bigger(
        fuzz_engine_builder: U256CompareFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            comparison_strategy(
                op(FunctionToTest::Lt256),
                EXPECTED_FALSE,
                arb_255bit_u256_be()
                    .prop_map(|b| (b + U256::one(), b))
                    .boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_compare_le_256_less(
        fuzz_engine_builder: U256CompareFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            comparison_strategy(
                op(FunctionToTest::Le256),
                EXPECTED_TRUE,
                arb_255bit_u256_be()
                    .prop_map(|a| (a, a + U256::one()))
                    .boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_compare_le_256_eq(
        fuzz_engine_builder: U256CompareFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            comparison_strategy(
                op(FunctionToTest::Le256),
                EXPECTED_TRUE,
                arb_u256_be().prop_map(|value| (value, value)).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_compare_le_256_bigger(
        fuzz_engine_builder: U256CompareFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            comparison_strategy(
                op(FunctionToTest::Le256),
                EXPECTED_FALSE,
                arb_255bit_u256_be()
                    .prop_map(|b| (b + U256::one(), b))
                    .boxed(),
            ),
        )
    }
}
