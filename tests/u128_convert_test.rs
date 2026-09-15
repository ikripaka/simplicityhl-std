mod common;

use primitive_types::U256;
use rand::Rng;

use common::core::{Expect, run};

use simplicityhl_std::artifacts::u128_convert_test::U128ConvertTestProgram;
use simplicityhl_std::artifacts::u128_convert_test::derived_u128_convert_test::{
    U128ConvertTestArguments, U128ConvertTestWitness,
};

enum FunctionToTest {
    U128ToU256,
    SplitU128IntoU8,
    SplitU128IntoU16,
    SplitU128IntoU32,
    SplitU128IntoU64,
    SafeU128ToU1,
    SafeU128ToU8,
    SafeU128ToU16,
    SafeU128ToU32,
    SafeU128ToU64,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

fn program() -> U128ConvertTestProgram {
    U128ConvertTestProgram::new(U128ConvertTestArguments {})
}

fn build_witness(function: u8, a: u128, expected: [u8; 32]) -> U128ConvertTestWitness {
    U128ConvertTestWitness {
        function_index: function,
        first_arg: a,
        expected,
    }
}

mod u128_convert_test {
    use super::*;

    #[simplex::test]
    fn u128_convert_test_u128_to_u256(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U128ToU256),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_convert_test_split_u128_into_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU128IntoU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_convert_test_split_u128_into_u16(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU128IntoU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_convert_test_split_u128_into_u32(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU128IntoU32),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_convert_test_split_u128_into_u64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU128IntoU64),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_convert_test_safe_u128_to_u1(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU128ToU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_convert_test_safe_u128_to_u1_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(2..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU128ToU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u128_convert_test_safe_u128_to_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u8::MAX as u128);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU128ToU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_convert_test_safe_u128_to_u8_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(u8::MAX as u128 + 1..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU128ToU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u128_convert_test_safe_u128_to_u16(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u16::MAX as u128);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU128ToU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_convert_test_safe_u128_to_u16_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(u16::MAX as u128 + 1..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU128ToU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u128_convert_test_safe_u128_to_u32(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u32::MAX as u128);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU128ToU32),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_convert_test_safe_u128_to_u32_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(u32::MAX as u128 + 1..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU128ToU32),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u128_convert_test_safe_u128_to_u64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u64::MAX as u128);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU128ToU64),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_convert_test_safe_u128_to_u64_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(u64::MAX as u128 + 1..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU128ToU64),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }
}

mod u128_convert_test_fuzz {
    use super::*;

    use common::core::FuzzExecutionCheck;
    use simplex::fuzz;
    use simplex::fuzz::builders::{FinalTransactionBuilder, ProgramTarget};
    use simplex::fuzz::engine::FuzzStrategyBuilder;
    use simplex::fuzz::proptest::prelude::any;
    use simplex::fuzz::proptest::strategy::{BoxedStrategy, Strategy};
    use simplex::fuzz::{FuzzEngineBuilder, FuzzError};
    use simplex::simplicityhl::{Arguments, WitnessValues};
    use simplex::transaction::{FinalTransaction, PartialInput, RequiredSignature, UTXO};

    const PROGRAM_TARGET: ProgramTarget = ProgramTarget::Input(0);

    type U128ConvertFuzzEngineBuilder =
        FuzzEngineBuilder<U128ConvertTestProgram, U128ConvertTestArguments, U128ConvertTestWitness>;

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

    fn arb_u1_u128() -> impl Strategy<Value = u128> {
        any::<bool>().prop_map(u128::from)
    }

    fn arb_u8_u128() -> impl Strategy<Value = u128> {
        any::<u8>().prop_map(u128::from)
    }

    fn arb_u16_u128() -> impl Strategy<Value = u128> {
        any::<u16>().prop_map(u128::from)
    }

    fn arb_u32_u128() -> impl Strategy<Value = u128> {
        any::<u32>().prop_map(u128::from)
    }

    fn arb_u64_u128() -> impl Strategy<Value = u128> {
        any::<u64>().prop_map(u128::from)
    }

    fn arb_u128_larger_than(max: u128) -> impl Strategy<Value = u128> {
        arb_u128().prop_filter("u128 should exceed the conversion maximum", move |value| {
            *value > max
        })
    }

    fn conversion_strategy(
        function_index: u8,
        inputs: BoxedStrategy<u128>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U128ConvertTestArguments, U128ConvertTestWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |input| {
                let arguments: Arguments = U128ConvertTestArguments {}.into();
                let witness: WitnessValues =
                    build_witness(function_index, input, U256::from(input).to_big_endian()).into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_conversion_fuzz(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
        expect: Expect,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u128 conversion", expect));

        Ok(())
    }

    #[simplex::fuzz]
    fn u128_convert_test_u128_to_u256(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U128ToU256), arb_u128().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_split_u128_into_u8(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU128IntoU8), arb_u128().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_split_u128_into_u16(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU128IntoU16), arb_u128().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_split_u128_into_u32(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU128IntoU32), arb_u128().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_split_u128_into_u64(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU128IntoU64), arb_u128().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_safe_u128_to_u1(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU128ToU1), arb_u1_u128().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_safe_u128_to_u1_overflow(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU128ToU1),
                arb_u128_larger_than(1).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_safe_u128_to_u8(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU128ToU8), arb_u8_u128().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_safe_u128_to_u8_overflow(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU128ToU8),
                arb_u128_larger_than(u8::MAX as u128).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_safe_u128_to_u16(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU128ToU16), arb_u16_u128().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_safe_u128_to_u16_overflow(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU128ToU16),
                arb_u128_larger_than(u16::MAX as u128).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_safe_u128_to_u32(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU128ToU32), arb_u32_u128().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_safe_u128_to_u32_overflow(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU128ToU32),
                arb_u128_larger_than(u32::MAX as u128).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_safe_u128_to_u64(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU128ToU64), arb_u64_u128().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u128_convert_test_safe_u128_to_u64_overflow(
        fuzz_engine_builder: U128ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU128ToU64),
                arb_u128_larger_than(u64::MAX as u128).boxed(),
            ),
            Expect::AssertFailed,
        )
    }
}
