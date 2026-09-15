mod common;

use primitive_types::U256;
use rand::Rng;

use common::core::{Expect, run};

use simplicityhl_std::artifacts::u16_convert_test::U16ConvertTestProgram;
use simplicityhl_std::artifacts::u16_convert_test::derived_u16_convert_test::{
    U16ConvertTestArguments, U16ConvertTestWitness,
};

enum FunctionToTest {
    U16ToU32,
    U16ToU64,
    U16ToU128,
    U16ToU256,
    SplitU16IntoU8,
    SafeU16ToU1,
    SafeU16ToU8,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

fn program() -> U16ConvertTestProgram {
    U16ConvertTestProgram::new(U16ConvertTestArguments {})
}

fn build_witness(function: u8, a: u16, expected: [u8; 32]) -> U16ConvertTestWitness {
    U16ConvertTestWitness {
        function_index: function,
        first_arg: a,
        expected,
    }
}

mod u16_convert_test {
    use super::*;

    #[simplex::test]
    fn u16_convert_test_u16_to_u32(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u16::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U16ToU32),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u16_convert_test_u16_to_u64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u16::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U16ToU64),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u16_convert_test_u16_to_u128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u16::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U16ToU128),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u16_convert_test_u16_to_u256(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u16::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U16ToU256),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u16_convert_test_split_u16_into_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u16::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU16IntoU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u16_convert_test_safe_u16_to_u1(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU16ToU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u16_convert_test_safe_u16_to_u1_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(2..=u16::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU16ToU1),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u16_convert_test_safe_u16_to_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u8::MAX as u16);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU16ToU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u16_convert_test_safe_u16_to_u8_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(u8::MAX as u16 + 1..=u16::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU16ToU8),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }
}

mod u16_convert_test_fuzz {
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

    type U16ConvertFuzzEngineBuilder =
        FuzzEngineBuilder<U16ConvertTestProgram, U16ConvertTestArguments, U16ConvertTestWitness>;

    fn initial_transaction() -> FinalTransaction {
        let mut tx = FinalTransaction::new();
        tx.add_input(PartialInput::new(UTXO::default()), RequiredSignature::None);
        tx
    }

    /// Create the single program input that the fuzz engine replaces for each case.
    fn transaction_builder() -> Result<FinalTransactionBuilder, FuzzError> {
        FinalTransactionBuilder::new(initial_transaction(), [PROGRAM_TARGET])
    }

    /// Checks whether executing the contract matches its expected result.
    fn arb_u16() -> impl Strategy<Value = u16> {
        any::<u16>()
    }

    fn arb_u1_u16() -> impl Strategy<Value = u16> {
        any::<bool>().prop_map(u16::from)
    }

    fn arb_u8_u16() -> impl Strategy<Value = u16> {
        any::<u8>().prop_map(u16::from)
    }

    fn arb_u16_larger_than(max: u16) -> impl Strategy<Value = u16> {
        arb_u16().prop_filter("u16 should exceed the conversion maximum", move |value| {
            *value > max
        })
    }

    fn conversion_strategy(
        function_index: u8,
        inputs: BoxedStrategy<u16>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U16ConvertTestArguments, U16ConvertTestWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |input| {
                let arguments: Arguments = U16ConvertTestArguments {}.into();
                let witness: WitnessValues =
                    build_witness(function_index, input, U256::from(input).to_big_endian()).into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_conversion_fuzz(
        fuzz_engine_builder: U16ConvertFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
        expect: Expect,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u16 conversion", expect));

        Ok(())
    }

    #[simplex::fuzz]
    fn u16_convert_test_u16_to_u32(
        fuzz_engine_builder: U16ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U16ToU32), arb_u16().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u16_convert_test_u16_to_u64(
        fuzz_engine_builder: U16ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U16ToU64), arb_u16().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u16_convert_test_u16_to_u128(
        fuzz_engine_builder: U16ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U16ToU128), arb_u16().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u16_convert_test_u16_to_u256(
        fuzz_engine_builder: U16ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U16ToU256), arb_u16().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u16_convert_test_split_u16_into_u8(
        fuzz_engine_builder: U16ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU16IntoU8), arb_u16().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u16_convert_test_safe_u16_to_u1(
        fuzz_engine_builder: U16ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU16ToU1), arb_u1_u16().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u16_convert_test_safe_u16_to_u1_overflow(
        fuzz_engine_builder: U16ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU16ToU1),
                arb_u16_larger_than(1).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u16_convert_test_safe_u16_to_u8(
        fuzz_engine_builder: U16ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU16ToU8), arb_u8_u16().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u16_convert_test_safe_u16_to_u8_overflow(
        fuzz_engine_builder: U16ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU16ToU8),
                arb_u16_larger_than(u8::MAX as u16).boxed(),
            ),
            Expect::AssertFailed,
        )
    }
}
