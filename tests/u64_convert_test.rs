mod common;

use primitive_types::U256;
use rand::Rng;

use common::core::{Expect, run};

use simplicityhl_std::artifacts::u64_convert_test::U64ConvertTestProgram;
use simplicityhl_std::artifacts::u64_convert_test::derived_u64_convert_test::{
    U64ConvertTestArguments, U64ConvertTestWitness,
};

enum FunctionToTest {
    U64ToU128,
    U64ToU256,
    SplitU64IntoU8,
    SplitU64IntoU16,
    SplitU64IntoU32,
    SafeU64ToU1,
    SafeU64ToU8,
    SafeU64ToU16,
    SafeU64ToU32,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

fn program() -> U64ConvertTestProgram {
    U64ConvertTestProgram::new(U64ConvertTestArguments {})
}

fn build_witness(function: u8, a: u64, expected: [u8; 32]) -> U64ConvertTestWitness {
    U64ConvertTestWitness {
        function_index: function,
        first_arg: a,
        expected,
    }
}

mod u64_convert_test {
    use super::*;

    #[simplex::test]
    fn u64_convert_test_u64_to_u128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u64::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U64ToU128),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u64_convert_test_u64_to_u256(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u64::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U64ToU256),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u64_convert_test_split_u64_into_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u64::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU64IntoU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u64_convert_test_split_u64_into_u16(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u64::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU64IntoU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u64_convert_test_split_u64_into_u32(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u64::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU64IntoU32),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u64_convert_test_safe_u64_to_u1(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU64ToU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u64_convert_test_safe_u64_to_u1_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(2..=u64::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU64ToU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u64_convert_test_safe_u64_to_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u8::MAX as u64);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU64ToU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u64_convert_test_safe_u64_to_u8_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(u8::MAX as u64 + 1..=u64::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU64ToU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u64_convert_test_safe_u64_to_u16(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u16::MAX as u64);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU64ToU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u64_convert_test_safe_u64_to_u16_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(u16::MAX as u64 + 1..=u64::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU64ToU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u64_convert_test_safe_u64_to_u32(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u32::MAX as u64);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU64ToU32),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u64_convert_test_safe_u64_to_u32_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(u32::MAX as u64 + 1..=u64::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU64ToU32),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }
}

mod u64_convert_test_fuzz {
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

    type U64ConvertFuzzEngineBuilder =
        FuzzEngineBuilder<U64ConvertTestProgram, U64ConvertTestArguments, U64ConvertTestWitness>;

    fn initial_transaction() -> FinalTransaction {
        let mut tx = FinalTransaction::new();
        tx.add_input(PartialInput::new(UTXO::default()), RequiredSignature::None);
        tx
    }

    fn transaction_builder() -> Result<FinalTransactionBuilder, FuzzError> {
        FinalTransactionBuilder::new(initial_transaction(), [PROGRAM_TARGET])
    }

    fn arb_u64() -> impl Strategy<Value = u64> {
        any::<u64>()
    }

    fn arb_u1_u64() -> impl Strategy<Value = u64> {
        any::<bool>().prop_map(u64::from)
    }

    fn arb_u8_u64() -> impl Strategy<Value = u64> {
        any::<u8>().prop_map(u64::from)
    }

    fn arb_u16_u64() -> impl Strategy<Value = u64> {
        any::<u16>().prop_map(u64::from)
    }

    fn arb_u32_u64() -> impl Strategy<Value = u64> {
        any::<u32>().prop_map(u64::from)
    }

    fn arb_u64_larger_than(max: u64) -> impl Strategy<Value = u64> {
        arb_u64().prop_filter("u64 should exceed the conversion maximum", move |value| {
            *value > max
        })
    }

    fn conversion_strategy(
        function_index: u8,
        inputs: BoxedStrategy<u64>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U64ConvertTestArguments, U64ConvertTestWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |input| {
                let arguments: Arguments = U64ConvertTestArguments {}.into();
                let witness: WitnessValues =
                    build_witness(function_index, input, U256::from(input).to_big_endian()).into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_conversion_fuzz(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
        expect: Expect,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u64 conversion", expect));

        Ok(())
    }

    #[simplex::fuzz]
    fn u64_convert_test_u64_to_u128(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U64ToU128), arb_u64().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_u64_to_u256(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U64ToU256), arb_u64().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_split_u64_into_u8(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU64IntoU8), arb_u64().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_split_u64_into_u16(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU64IntoU16), arb_u64().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_split_u64_into_u32(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU64IntoU32), arb_u64().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_safe_u64_to_u1(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU64ToU1), arb_u1_u64().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_safe_u64_to_u1_overflow(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU64ToU1),
                arb_u64_larger_than(1).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_safe_u64_to_u8(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU64ToU8), arb_u8_u64().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_safe_u64_to_u8_overflow(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU64ToU8),
                arb_u64_larger_than(u8::MAX as u64).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_safe_u64_to_u16(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU64ToU16), arb_u16_u64().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_safe_u64_to_u16_overflow(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU64ToU16),
                arb_u64_larger_than(u16::MAX as u64).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_safe_u64_to_u32(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU64ToU32), arb_u32_u64().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u64_convert_test_safe_u64_to_u32_overflow(
        fuzz_engine_builder: U64ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU64ToU32),
                arb_u64_larger_than(u32::MAX as u64).boxed(),
            ),
            Expect::AssertFailed,
        )
    }
}
