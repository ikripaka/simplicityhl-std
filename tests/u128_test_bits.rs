mod common;

use rand::Rng;

use crate::common::helper::DEFAULT_BOOL;
use common::core::{Expect, run};

use simplicityhl_std::artifacts::u128_test_bits::U128TestBitsProgram;
use simplicityhl_std::artifacts::u128_test_bits::derived_u128_test_bits::{
    U128TestBitsArguments, U128TestBitsWitness,
};

enum FunctionToTest {
    And128,
    Or128,
    Eq128,
    LeftShift128,
    RightShift128,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

const DEFAULT_EXPECTED: u128 = 0;

fn program() -> U128TestBitsProgram {
    U128TestBitsProgram::new(U128TestBitsArguments {})
}

fn build_witness(
    function: u8,
    a: u128,
    b: u128,
    expected: Option<u128>,
    expected_bool: bool,
) -> U128TestBitsWitness {
    U128TestBitsWitness {
        function_index: function,
        first_arg: a,
        second_arg: b,
        expected,
        expected_bool,
    }
}

mod u128_tests_bits {
    use super::*;

    #[simplex::test]
    fn u128_test_and_128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);
        let b = rand::thread_rng().gen_range(0..=u128::MAX);
        let result = a & b;

        run(
            &context,
            program(),
            build_witness(op(FunctionToTest::And128), a, b, Some(result), DEFAULT_BOOL),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_or_128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);
        let b = rand::thread_rng().gen_range(0..=u128::MAX);
        let result = a | b;

        run(
            &context,
            program(),
            build_witness(op(FunctionToTest::Or128), a, b, Some(result), DEFAULT_BOOL),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_eq_128_true(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Eq128),
                a,
                a,
                Some(DEFAULT_EXPECTED),
                true,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_eq_128_false(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(1..=u128::MAX);
        let b = a - 1;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Eq128),
                a,
                b,
                Some(DEFAULT_EXPECTED),
                false,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_left_shift_128(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = rand::thread_rng().gen_range(1..=127_u128);
        let val = rand::thread_rng().gen_range(0..=u128::MAX);
        let result = val << shift;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::LeftShift128),
                shift,
                val,
                Some(result),
                DEFAULT_BOOL,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_left_shift_128_by_zero(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = 0;
        let val = rand::thread_rng().gen_range(0..=u128::MAX);
        let result = val;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::LeftShift128),
                shift,
                val,
                Some(result),
                DEFAULT_BOOL,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_left_shift_128_out_of_range(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = rand::thread_rng().gen_range(128..=u8::MAX as u128);
        let val = rand::thread_rng().gen_range(0..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::LeftShift128),
                shift,
                val,
                Some(0),
                DEFAULT_BOOL,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_right_shift_128(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = rand::thread_rng().gen_range(1..=127_u128);
        let val = rand::thread_rng().gen_range(0..=u128::MAX);
        let result = val >> shift;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::RightShift128),
                shift,
                val,
                Some(result),
                DEFAULT_BOOL,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_right_shift_128_by_zero(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = 0;
        let val = rand::thread_rng().gen_range(0..=u128::MAX);
        let result = val;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::RightShift128),
                shift,
                val,
                Some(result),
                DEFAULT_BOOL,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_right_shift_128_out_of_range(context: simplex::TestContext) -> anyhow::Result<()> {
        let shift = rand::thread_rng().gen_range(128..=u8::MAX as u128);
        let val = rand::thread_rng().gen_range(0..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::RightShift128),
                shift,
                val,
                Some(0),
                DEFAULT_BOOL,
            ),
            Expect::Ok,
        )
    }
}

mod u128_tests_bits_fuzz {
    use super::*;

    use common::core::FuzzExecutionCheck;
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

    type U128BitFuzzEngineBuilder =
        FuzzEngineBuilder<U128TestBitsProgram, U128TestBitsArguments, U128TestBitsWitness>;

    fn initial_transaction() -> FinalTransaction {
        let mut tx = FinalTransaction::new();
        tx.add_input(PartialInput::new(UTXO::default()), RequiredSignature::None);
        tx
    }

    fn transaction_builder() -> Result<FinalTransactionBuilder, FuzzError> {
        FinalTransactionBuilder::new(initial_transaction(), [PROGRAM_TARGET])
    }

    fn arb_u128() -> impl Strategy<Value = u128> {
        any::<u128>()
    }

    fn arb_non_zero_u128() -> impl Strategy<Value = u128> {
        arb_u128().prop_filter("u128 should not be zero", |value| *value != 0)
    }

    fn arb_non_zero_in_range_shift_u8() -> impl Strategy<Value = u8> {
        any::<u8>().prop_filter("shift should be in 1..128", |shift| {
            *shift > 0 && *shift < 128
        })
    }

    fn arb_out_of_range_shift_u8() -> impl Strategy<Value = u8> {
        any::<u8>().prop_filter("shift should be in 128..=255", |shift| *shift >= 128)
    }

    fn bitwise_strategy(
        function_index: u8,
        expected_bool: bool,
        inputs: BoxedStrategy<(u128, u128)>,
        operation: fn(u128, u128) -> u128,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U128TestBitsArguments, U128TestBitsWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(a, b)| {
                let arguments: Arguments = U128TestBitsArguments {}.into();
                let witness: WitnessValues =
                    build_witness(function_index, a, b, Some(operation(a, b)), expected_bool)
                        .into();

                (arguments, witness)
            }))
            .build()
    }

    fn equality_strategy(
        expected_bool: bool,
        inputs: BoxedStrategy<(u128, u128)>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U128TestBitsArguments, U128TestBitsWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(a, b)| {
                let arguments: Arguments = U128TestBitsArguments {}.into();
                let witness: WitnessValues = build_witness(
                    op(FunctionToTest::Eq128),
                    a,
                    b,
                    Some(DEFAULT_EXPECTED),
                    expected_bool,
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn shift_strategy(
        function_index: u8,
        expected_bool: bool,
        inputs: BoxedStrategy<(u128, u8)>,
        operation: fn(u128, u8) -> u128,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U128TestBitsArguments, U128TestBitsWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(value, shift)| {
                let arguments: Arguments = U128TestBitsArguments {}.into();
                let witness: WitnessValues = build_witness(
                    function_index,
                    u128::from(shift),
                    value,
                    Some(operation(value, shift)),
                    expected_bool,
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn out_of_range_shift_strategy(
        function_index: u8,
        expected_bool: bool,
        inputs: BoxedStrategy<(u128, u8)>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U128TestBitsArguments, U128TestBitsWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(value, shift)| {
                let arguments: Arguments = U128TestBitsArguments {}.into();
                let witness: WitnessValues = build_witness(
                    function_index,
                    u128::from(shift),
                    value,
                    Some(DEFAULT_EXPECTED),
                    expected_bool,
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_successful_fuzz(
        fuzz_engine_builder: U128BitFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u128 bit operation", Expect::Ok));

        Ok(())
    }

    #[simplex::fuzz]
    fn u128_test_bits_and_128(fuzz_engine_builder: U128BitFuzzEngineBuilder) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            bitwise_strategy(
                op(FunctionToTest::And128),
                EXPECTED_FALSE,
                (arb_u128(), arb_u128()).boxed(),
                |a, b| a & b,
            ),
        )
    }

    #[simplex::fuzz]
    fn u128_test_bits_or_128(fuzz_engine_builder: U128BitFuzzEngineBuilder) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            bitwise_strategy(
                op(FunctionToTest::Or128),
                EXPECTED_FALSE,
                (arb_u128(), arb_u128()).boxed(),
                |a, b| a | b,
            ),
        )
    }

    #[simplex::fuzz]
    fn u128_test_bits_eq_128_true(
        fuzz_engine_builder: U128BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            equality_strategy(
                EXPECTED_TRUE,
                arb_u128().prop_map(|value| (value, value)).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u128_test_bits_eq_128_false(
        fuzz_engine_builder: U128BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            equality_strategy(
                EXPECTED_FALSE,
                arb_non_zero_u128().prop_map(|a| (a, a - 1)).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u128_test_bits_left_shift_128(
        fuzz_engine_builder: U128BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            shift_strategy(
                op(FunctionToTest::LeftShift128),
                EXPECTED_FALSE,
                (arb_u128(), arb_non_zero_in_range_shift_u8()).boxed(),
                |value, shift| value << shift,
            ),
        )
    }

    #[simplex::fuzz]
    fn u128_test_bits_left_shift_128_by_zero(
        fuzz_engine_builder: U128BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            shift_strategy(
                op(FunctionToTest::LeftShift128),
                EXPECTED_FALSE,
                (arb_u128(), Just(0_u8)).boxed(),
                |value, shift| value << shift,
            ),
        )
    }

    #[simplex::fuzz]
    fn u128_test_bits_left_shift_128_out_of_range(
        fuzz_engine_builder: U128BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            out_of_range_shift_strategy(
                op(FunctionToTest::LeftShift128),
                EXPECTED_FALSE,
                (arb_u128(), arb_out_of_range_shift_u8()).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u128_test_bits_right_shift_128(
        fuzz_engine_builder: U128BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            shift_strategy(
                op(FunctionToTest::RightShift128),
                EXPECTED_FALSE,
                (arb_u128(), arb_non_zero_in_range_shift_u8()).boxed(),
                |value, shift| value >> shift,
            ),
        )
    }

    #[simplex::fuzz]
    fn u128_test_bits_right_shift_128_by_zero(
        fuzz_engine_builder: U128BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            shift_strategy(
                op(FunctionToTest::RightShift128),
                EXPECTED_FALSE,
                (arb_u128(), Just(0_u8)).boxed(),
                |value, shift| value >> shift,
            ),
        )
    }

    #[simplex::fuzz]
    fn u128_test_bits_right_shift_128_out_of_range(
        fuzz_engine_builder: U128BitFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            out_of_range_shift_strategy(
                op(FunctionToTest::RightShift128),
                EXPECTED_FALSE,
                (arb_u128(), arb_out_of_range_shift_u8()).boxed(),
            ),
        )
    }
}
