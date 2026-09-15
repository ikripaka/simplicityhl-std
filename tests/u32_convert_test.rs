mod common;

use primitive_types::U256;
use rand::Rng;

use common::core::{Expect, run};

use simplicityhl_std::artifacts::u32_convert_test::U32ConvertTestProgram;
use simplicityhl_std::artifacts::u32_convert_test::derived_u32_convert_test::{
    U32ConvertTestArguments, U32ConvertTestWitness,
};

enum FunctionToTest {
    U32ToU64,
    U32ToU128,
    U32ToU256,
    SplitU32IntoU8,
    SplitU32IntoU16,
    SafeU32ToU1,
    SafeU32ToU8,
    SafeU32ToU16,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

fn program() -> U32ConvertTestProgram {
    U32ConvertTestProgram::new(U32ConvertTestArguments {})
}

fn build_witness(function: u8, a: u32, expected: [u8; 32]) -> U32ConvertTestWitness {
    U32ConvertTestWitness {
        function_index: function,
        first_arg: a,
        expected,
    }
}

mod u32_convert_test {
    use super::*;

    #[simplex::test]
    fn u32_convert_test_u32_to_u64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u32::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U32ToU64),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u32_convert_test_u32_to_u128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u32::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U32ToU128),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u32_convert_test_u32_to_u256(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u32::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U32ToU256),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u32_convert_test_split_u32_into_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u32::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU32IntoU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u32_convert_test_split_u32_into_u16(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u32::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU32IntoU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u32_convert_test_safe_u32_to_u1(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU32ToU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u32_convert_test_safe_u32_to_u1_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(2..=u32::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU32ToU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u32_convert_test_safe_u32_to_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u8::MAX as u32);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU32ToU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u32_convert_test_safe_u32_to_u8_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(u8::MAX as u32 + 1..=u32::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU32ToU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u32_convert_test_safe_u32_to_u16(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u16::MAX as u32);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU32ToU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u32_convert_test_safe_u32_to_u16_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(u16::MAX as u32 + 1..=u32::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU32ToU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }
}

mod u32_convert_test_fuzz {
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

    type U32ConvertFuzzEngineBuilder =
        FuzzEngineBuilder<U32ConvertTestProgram, U32ConvertTestArguments, U32ConvertTestWitness>;

    fn initial_transaction() -> FinalTransaction {
        let mut tx = FinalTransaction::new();
        tx.add_input(PartialInput::new(UTXO::default()), RequiredSignature::None);
        tx
    }

    fn transaction_builder() -> Result<FinalTransactionBuilder, FuzzError> {
        FinalTransactionBuilder::new(initial_transaction(), [PROGRAM_TARGET])
    }

    fn arb_u32() -> impl Strategy<Value = u32> {
        any::<u32>()
    }

    fn arb_u1_u32() -> impl Strategy<Value = u32> {
        any::<bool>().prop_map(u32::from)
    }

    fn arb_u8_u32() -> impl Strategy<Value = u32> {
        any::<u8>().prop_map(u32::from)
    }

    fn arb_u16_u32() -> impl Strategy<Value = u32> {
        any::<u16>().prop_map(u32::from)
    }

    fn arb_u32_larger_than(max: u32) -> impl Strategy<Value = u32> {
        arb_u32().prop_filter("u32 should exceed the conversion maximum", move |value| {
            *value > max
        })
    }

    fn conversion_strategy(
        function_index: u8,
        inputs: BoxedStrategy<u32>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U32ConvertTestArguments, U32ConvertTestWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |input| {
                let arguments: Arguments = U32ConvertTestArguments {}.into();
                let witness: WitnessValues =
                    build_witness(function_index, input, U256::from(input).to_big_endian()).into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_conversion_fuzz(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
        expect: Expect,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u32 conversion", expect));

        Ok(())
    }

    #[simplex::fuzz]
    fn u32_convert_test_u32_to_u64(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U32ToU64), arb_u32().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u32_convert_test_u32_to_u128(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U32ToU128), arb_u32().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u32_convert_test_u32_to_u256(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U32ToU256), arb_u32().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u32_convert_test_split_u32_into_u8(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU32IntoU8), arb_u32().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u32_convert_test_split_u32_into_u16(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU32IntoU16), arb_u32().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u32_convert_test_safe_u32_to_u1(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU32ToU1), arb_u1_u32().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u32_convert_test_safe_u32_to_u1_overflow(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU32ToU1),
                arb_u32_larger_than(1).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u32_convert_test_safe_u32_to_u8(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU32ToU8), arb_u8_u32().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u32_convert_test_safe_u32_to_u8_overflow(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU32ToU8),
                arb_u32_larger_than(u8::MAX as u32).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u32_convert_test_safe_u32_to_u16(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU32ToU16), arb_u16_u32().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u32_convert_test_safe_u32_to_u16_overflow(
        fuzz_engine_builder: U32ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU32ToU16),
                arb_u32_larger_than(u16::MAX as u32).boxed(),
            ),
            Expect::AssertFailed,
        )
    }
}
