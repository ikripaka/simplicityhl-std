mod common;

use primitive_types::U256;
use rand::Rng;

use common::core::{Expect, run};

use simplicityhl_std::artifacts::u8_convert_test::U8ConvertTestProgram;
use simplicityhl_std::artifacts::u8_convert_test::derived_u8_convert_test::{
    U8ConvertTestArguments, U8ConvertTestWitness,
};

enum FunctionToTest {
    U8ToU16,
    U8ToU32,
    U8ToU64,
    U8ToU128,
    U8ToU256,
    SplitU8IntoU1,
    SafeU8ToU1,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

fn program() -> U8ConvertTestProgram {
    U8ConvertTestProgram::new(U8ConvertTestArguments {})
}

fn build_witness(function: u8, a: u8, expected: [u8; 32]) -> U8ConvertTestWitness {
    U8ConvertTestWitness {
        function_index: function,
        first_arg: a,
        expected,
    }
}

mod u8_convert_test {
    use super::*;

    #[simplex::test]
    fn u8_convert_test_u8_to_u16(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u8::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U8ToU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u8_convert_test_u8_to_u32(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u8::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U8ToU32),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u8_convert_test_u8_to_u64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u8::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U8ToU64),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u8_convert_test_u8_to_u128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u8::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U8ToU128),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u8_convert_test_u8_to_u256(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u8::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U8ToU256),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u8_convert_test_split_u8_into_u1(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u8::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU8IntoU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u8_convert_test_safe_u8_to_u1(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU8ToU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u8_convert_test_safe_u8_to_u1_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(2..=u8::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU8ToU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }
}

mod u8_convert_test_fuzz {
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

    type U8ConvertFuzzEngineBuilder =
        FuzzEngineBuilder<U8ConvertTestProgram, U8ConvertTestArguments, U8ConvertTestWitness>;

    fn initial_transaction() -> FinalTransaction {
        let mut tx = FinalTransaction::new();
        tx.add_input(PartialInput::new(UTXO::default()), RequiredSignature::None);
        tx
    }

    /// Create the single program input that the fuzz engine replaces for each case.
    fn transaction_builder() -> Result<FinalTransactionBuilder, FuzzError> {
        FinalTransactionBuilder::new(initial_transaction(), [PROGRAM_TARGET])
    }

    fn arb_u8() -> impl Strategy<Value = u8> {
        any::<u8>()
    }

    fn arb_u1_u8() -> impl Strategy<Value = u8> {
        any::<bool>().prop_map(u8::from)
    }

    fn arb_u8_larger_than(max: u8) -> impl Strategy<Value = u8> {
        arb_u8().prop_filter("u8 should exceed the conversion maximum", move |value| {
            *value > max
        })
    }

    fn conversion_strategy(
        function_index: u8,
        inputs: BoxedStrategy<u8>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U8ConvertTestArguments, U8ConvertTestWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |input| {
                let arguments: Arguments = U8ConvertTestArguments {}.into();
                let witness: WitnessValues =
                    build_witness(function_index, input, U256::from(input).to_big_endian()).into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_conversion_fuzz(
        fuzz_engine_builder: U8ConvertFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
        expect: Expect,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u8 conversion", expect));

        Ok(())
    }

    #[simplex::fuzz]
    fn u8_convert_test_u8_to_u16(
        fuzz_engine_builder: U8ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U8ToU16), arb_u8().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u8_convert_test_u8_to_u32(
        fuzz_engine_builder: U8ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U8ToU32), arb_u8().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u8_convert_test_u8_to_u64(
        fuzz_engine_builder: U8ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U8ToU64), arb_u8().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u8_convert_test_u8_to_u128(
        fuzz_engine_builder: U8ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U8ToU128), arb_u8().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u8_convert_test_u8_to_u256(
        fuzz_engine_builder: U8ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U8ToU256), arb_u8().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u8_convert_test_split_u8_into_u1(
        fuzz_engine_builder: U8ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU8IntoU1), arb_u8().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u8_convert_test_safe_u8_to_u1(
        fuzz_engine_builder: U8ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU8ToU1), arb_u1_u8().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u8_convert_test_safe_u8_to_u1_overflow(
        fuzz_engine_builder: U8ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU8ToU1),
                arb_u8_larger_than(1).boxed(),
            ),
            Expect::AssertFailed,
        )
    }
}
