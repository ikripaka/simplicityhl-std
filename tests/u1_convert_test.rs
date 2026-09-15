mod common;

use primitive_types::U256;
use rand::Rng;

use common::core::{Expect, run};

use simplicityhl_std::artifacts::u1_convert_test::U1ConvertTestProgram;
use simplicityhl_std::artifacts::u1_convert_test::derived_u1_convert_test::{
    U1ConvertTestArguments, U1ConvertTestWitness,
};

enum FunctionToTest {
    U1ToU8,
    U1ToU16,
    U1ToU32,
    U1ToU64,
    U1ToU128,
    U1ToU256,
    U1ToBool,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

fn program() -> U1ConvertTestProgram {
    U1ConvertTestProgram::new(U1ConvertTestArguments {})
}

fn build_witness(function: u8, a: u8, expected: [u8; 32]) -> U1ConvertTestWitness {
    U1ConvertTestWitness {
        function_index: function,
        first_arg: a,
        expected,
    }
}

mod u1_convert_test {
    use super::*;

    #[simplex::test]
    fn u1_convert_test_u1_to_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(op(FunctionToTest::U1ToU8), a, U256::from(a).to_big_endian()),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u1_convert_test_u1_to_u16(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U1ToU16),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u1_convert_test_u1_to_u32(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U1ToU32),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u1_convert_test_u1_to_u64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U1ToU64),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u1_convert_test_u1_to_u128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U1ToU128),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u1_convert_test_u1_to_u256(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U1ToU256),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u1_convert_test_split_u1_to_u1(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::U1ToBool),
                a,
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }
}

mod u1_convert_test_fuzz {
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

    type U1ConvertFuzzEngineBuilder =
        FuzzEngineBuilder<U1ConvertTestProgram, U1ConvertTestArguments, U1ConvertTestWitness>;

    fn initial_transaction() -> FinalTransaction {
        let mut tx = FinalTransaction::new();
        tx.add_input(PartialInput::new(UTXO::default()), RequiredSignature::None);
        tx
    }

    /// Create the single program input that the fuzz engine replaces for each case.
    fn transaction_builder() -> Result<FinalTransactionBuilder, FuzzError> {
        FinalTransactionBuilder::new(initial_transaction(), [PROGRAM_TARGET])
    }

    fn arb_u1() -> impl Strategy<Value = u8> {
        any::<bool>().prop_map(u8::from)
    }

    fn conversion_strategy(
        function_index: u8,
        inputs: BoxedStrategy<u8>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U1ConvertTestArguments, U1ConvertTestWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |input| {
                let arguments: Arguments = U1ConvertTestArguments {}.into();
                let witness: WitnessValues =
                    build_witness(function_index, input, U256::from(input).to_big_endian()).into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_conversion_fuzz(
        fuzz_engine_builder: U1ConvertFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
        expect: Expect,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u1 conversion", expect));

        Ok(())
    }

    #[simplex::fuzz]
    fn u1_convert_test_u1_to_u8(
        fuzz_engine_builder: U1ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U1ToU8), arb_u1().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u1_convert_test_u1_to_u16(
        fuzz_engine_builder: U1ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U1ToU16), arb_u1().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u1_convert_test_u1_to_u32(
        fuzz_engine_builder: U1ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U1ToU32), arb_u1().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u1_convert_test_u1_to_u64(
        fuzz_engine_builder: U1ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U1ToU64), arb_u1().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u1_convert_test_u1_to_u128(
        fuzz_engine_builder: U1ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U1ToU128), arb_u1().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u1_convert_test_u1_to_u256(
        fuzz_engine_builder: U1ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U1ToU256), arb_u1().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u1_convert_test_split_u1_to_u1(
        fuzz_engine_builder: U1ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::U1ToBool), arb_u1().boxed()),
            Expect::Ok,
        )
    }
}
