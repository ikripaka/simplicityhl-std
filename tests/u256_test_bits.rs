mod common;

use primitive_types::U256;
use rand::Rng;

use crate::common::helper::generate_u256;
use common::core::{Expect, run};

use simplicityhl_std::artifacts::u256_test_bits::U256TestBitsProgram;
use simplicityhl_std::artifacts::u256_test_bits::derived_u256_test_bits::{
    U256TestBitsArguments, U256TestBitsWitness,
};

enum FunctionToTest {
    And256,
    Or256,
    LeftShift256,
    RightShift256,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

fn program() -> U256TestBitsProgram {
    U256TestBitsProgram::new(U256TestBitsArguments {})
}

fn build_witness(
    function: u8,
    a: [u8; 32],
    b: [u8; 32],
    expected: Option<[u8; 32]>,
) -> U256TestBitsWitness {
    U256TestBitsWitness {
        function_index: function,
        first_arg: a,
        second_arg: b,
        expected,
    }
}

mod u256_tests_bits {
    use super::*;

    #[simplex::test]
    fn u256_test_and_256(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b = generate_u256(U256::zero(), U256::MAX);
        let result = (a & b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::And256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_or_256(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b = generate_u256(U256::zero(), U256::MAX);
        let result = (a | b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Or256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_left_shift_256(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = rand::thread_rng().gen_range(1..u8::MAX);
        let val = generate_u256(U256::zero(), U256::MAX);
        let result = (val << shift).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::LeftShift256),
                U256::from(shift).to_big_endian(),
                val.to_big_endian(),
                Some(result),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_left_shift_256_by_zero(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = 0;
        let val = generate_u256(U256::zero(), U256::MAX).to_big_endian();
        let result = val;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::LeftShift256),
                U256::from(shift).to_big_endian(),
                val,
                Some(result),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_left_shift_256_max(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = u8::MAX;
        let val = generate_u256(U256::zero(), U256::MAX);
        let result = (val << shift).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::LeftShift256),
                U256::from(shift).to_big_endian(),
                val.to_big_endian(),
                Some(result),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_right_shift_256(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = rand::thread_rng().gen_range(1..u8::MAX);
        let val = generate_u256(U256::zero(), U256::MAX);
        let result = (val >> shift).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::RightShift256),
                U256::from(shift).to_big_endian(),
                val.to_big_endian(),
                Some(result),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_right_shift_256_by_zero(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = 0;
        let val = generate_u256(U256::zero(), U256::MAX).to_big_endian();
        let result = val;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::RightShift256),
                U256::from(shift).to_big_endian(),
                val,
                Some(result),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_right_shift_256_max(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = u8::MAX;
        let val = generate_u256(U256::zero(), U256::MAX);
        let result = (val >> shift).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::RightShift256),
                U256::from(shift).to_big_endian(),
                val.to_big_endian(),
                Some(result),
            ),
            Expect::Ok,
        )
    }
}
mod u256_tests_bits_fuzz {
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

    type U256BitFuzzEngineBuilder =
        FuzzEngineBuilder<U256TestBitsProgram, U256TestBitsArguments, U256TestBitsWitness>;

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

    fn arb_non_zero_u8() -> impl Strategy<Value = u8> {
        any::<u8>().prop_filter("u8 should not be zero", |value| *value != 0)
    }

    fn bitwise_strategy(
        function_index: u8,
        inputs: BoxedStrategy<(U256, U256)>,
        operation: fn(U256, U256) -> U256,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U256TestBitsArguments, U256TestBitsWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(a, b)| {
                let expected = operation(a, b).to_big_endian();
                let arguments: Arguments = U256TestBitsArguments {}.into();
                let witness: WitnessValues = build_witness(
                    function_index,
                    a.to_big_endian(),
                    b.to_big_endian(),
                    Some(expected),
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn shift_strategy(
        function_index: u8,
        inputs: BoxedStrategy<(U256, u8)>,
        operation: fn(U256, u8) -> U256,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U256TestBitsArguments, U256TestBitsWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(value, shift)| {
                let expected = operation(value, shift).to_big_endian();
                let arguments: Arguments = U256TestBitsArguments {}.into();
                let witness: WitnessValues = build_witness(
                    function_index,
                    U256::from(shift).to_big_endian(),
                    value.to_big_endian(),
                    Some(expected),
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_successful_fuzz(
        fuzz_engine_builder: U256BitFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u256 bit operation", Expect::Ok));

        Ok(())
    }

    #[simplex::fuzz]
    fn u256_test_and_256(fuzz_engine_builder: U256BitFuzzEngineBuilder) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            bitwise_strategy(
                op(FunctionToTest::And256),
                (arb_u256_be(), arb_u256_be()).boxed(),
                |a, b| a & b,
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_or_256(fuzz_engine_builder: U256BitFuzzEngineBuilder) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            bitwise_strategy(
                op(FunctionToTest::Or256),
                (arb_u256_be(), arb_u256_be()).boxed(),
                |a, b| a | b,
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_left_shift_256(
        fuzz_engine_builder: U256BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            shift_strategy(
                op(FunctionToTest::LeftShift256),
                (arb_u256_be(), arb_non_zero_u8()).boxed(),
                |value, shift| value << shift,
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_left_shift_256_by_zero(
        fuzz_engine_builder: U256BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            shift_strategy(
                op(FunctionToTest::LeftShift256),
                (arb_u256_be(), Just(0_u8)).boxed(),
                |value, shift| value << shift,
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_left_shift_256_max(
        fuzz_engine_builder: U256BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            shift_strategy(
                op(FunctionToTest::LeftShift256),
                (arb_u256_be(), Just(u8::MAX)).boxed(),
                |value, shift| value << shift,
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_right_shift_256(
        fuzz_engine_builder: U256BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            shift_strategy(
                op(FunctionToTest::RightShift256),
                (arb_u256_be(), arb_non_zero_u8()).boxed(),
                |value, shift| value >> shift,
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_right_shift_256_by_zero(
        fuzz_engine_builder: U256BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            shift_strategy(
                op(FunctionToTest::RightShift256),
                (arb_u256_be(), Just(0_u8)).boxed(),
                |value, shift| value >> shift,
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_right_shift_256_max(
        fuzz_engine_builder: U256BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            shift_strategy(
                op(FunctionToTest::RightShift256),
                (arb_u256_be(), Just(u8::MAX)).boxed(),
                |value, shift| value >> shift,
            ),
        )
    }
}
