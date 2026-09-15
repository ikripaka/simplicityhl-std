mod common;

use primitive_types::U256;

use common::core::{Expect, run};

use simplicityhl_std::artifacts::u256_convert_test::U256ConvertTestProgram;
use simplicityhl_std::artifacts::u256_convert_test::derived_u256_convert_test::{
    U256ConvertTestArguments, U256ConvertTestWitness,
};

enum FunctionToTest {
    SplitU256IntoU8,
    SplitU256IntoU16,
    SplitU256IntoU32,
    SplitU256IntoU64,
    SplitU256IntoU128,
    SafeU256ToU1,
    SafeU256ToU8,
    SafeU256ToU16,
    SafeU256ToU32,
    SafeU256ToU64,
    SafeU256ToU128,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

fn program() -> U256ConvertTestProgram {
    U256ConvertTestProgram::new(U256ConvertTestArguments {})
}

fn build_witness(function: u8, a: [u8; 32], expected: [u8; 32]) -> U256ConvertTestWitness {
    U256ConvertTestWitness {
        function_index: function,
        first_arg: a,
        expected,
    }
}

mod u256_convert_test {
    use crate::common::helper::generate_u256;

    use super::*;

    #[simplex::test]
    fn u256_convert_test_u256_into_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU256IntoU8),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_u256_into_u16(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU256IntoU16),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_u256_into_u32(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU256IntoU32),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_u256_into_u64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU256IntoU64),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_u256_into_u128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SplitU256IntoU128),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u1(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::one());

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU1),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u1_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = generate_u256(U256::from(2), U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU1),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u8(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::from(u8::MAX));

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU8),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u8_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = generate_u256(U256::from(u8::MAX) + 1, U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU8),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u16(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::from(u16::MAX));

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU16),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u16_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = generate_u256(U256::from(u16::MAX) + 1, U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU16),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u32(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::from(u32::MAX));

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU32),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u32_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = generate_u256(U256::from(u32::MAX) + 1, U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU32),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::from(u64::MAX));

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU64),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u64_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = generate_u256(U256::from(u64::MAX) + 1, U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU64),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::from(u128::MAX));

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU128),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_convert_test_safe_u256_to_u128_overflow(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = generate_u256(U256::from(u128::MAX) + 1, U256::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeU256ToU128),
                a.to_big_endian(),
                U256::from(a).to_big_endian(),
            ),
            Expect::AssertFailed,
        )
    }
}

mod u256_convert_test_fuzz {
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

    type U256ConvertFuzzEngineBuilder =
        FuzzEngineBuilder<U256ConvertTestProgram, U256ConvertTestArguments, U256ConvertTestWitness>;

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

    fn arb_u1_u256_be() -> impl Strategy<Value = U256> {
        any::<bool>().prop_map(|value| U256::from(u8::from(value)))
    }

    fn arb_u8() -> impl Strategy<Value = u8> {
        any::<u8>()
    }

    fn arb_u8_u256_be() -> impl Strategy<Value = U256> {
        arb_u8().prop_map(U256::from)
    }

    fn arb_u16() -> impl Strategy<Value = u16> {
        any::<u16>()
    }

    fn arb_u16_u256_be() -> impl Strategy<Value = U256> {
        arb_u16().prop_map(U256::from)
    }

    fn arb_u32() -> impl Strategy<Value = u32> {
        any::<u32>()
    }

    fn arb_u32_u256_be() -> impl Strategy<Value = U256> {
        arb_u32().prop_map(U256::from)
    }

    fn arb_u64() -> impl Strategy<Value = u64> {
        any::<u64>()
    }

    fn arb_u64_u256_be() -> impl Strategy<Value = U256> {
        arb_u64().prop_map(U256::from)
    }

    fn arb_u128() -> impl Strategy<Value = u128> {
        any::<u128>()
    }

    fn arb_u128_u256_be() -> impl Strategy<Value = U256> {
        arb_u128().prop_map(U256::from)
    }

    fn arb_u256_larger_than(max: U256) -> impl Strategy<Value = U256> {
        arb_u256_be().prop_filter("u256 should exceed the conversion maximum", move |value| {
            *value > max
        })
    }

    fn conversion_strategy(
        function_index: u8,
        inputs: BoxedStrategy<U256>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U256ConvertTestArguments, U256ConvertTestWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |input| {
                let arguments: Arguments = U256ConvertTestArguments {}.into();
                let witness: WitnessValues =
                    build_witness(function_index, input.to_big_endian(), input.to_big_endian())
                        .into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_conversion_fuzz(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
        expect: Expect,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u256 conversion", expect));

        Ok(())
    }

    #[simplex::fuzz]
    fn u256_convert_test_u256_into_u8(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU256IntoU8), arb_u256_be().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_u256_into_u16(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU256IntoU16), arb_u256_be().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_u256_into_u32(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU256IntoU32), arb_u256_be().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_u256_into_u64(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU256IntoU64), arb_u256_be().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_u256_into_u128(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SplitU256IntoU128), arb_u256_be().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u1(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU256ToU1), arb_u1_u256_be().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u1_overflow(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU256ToU1),
                arb_u256_larger_than(U256::from(1)).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u8(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU256ToU8), arb_u8_u256_be().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u8_overflow(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU256ToU8),
                arb_u256_larger_than(U256::from(u8::MAX)).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u16(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU256ToU16), arb_u16_u256_be().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u16_overflow(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU256ToU16),
                arb_u256_larger_than(U256::from(u16::MAX)).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u32(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU256ToU32), arb_u32_u256_be().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u32_overflow(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU256ToU32),
                arb_u256_larger_than(u32::MAX.into()).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u64(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(op(FunctionToTest::SafeU256ToU64), arb_u64_u256_be().boxed()),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u64_overflow(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU256ToU64),
                arb_u256_larger_than(u64::MAX.into()).boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u128(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU256ToU128),
                arb_u128_u256_be().boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_convert_test_safe_u256_to_u128_overflow(
        fuzz_engine_builder: U256ConvertFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_conversion_fuzz(
            fuzz_engine_builder,
            conversion_strategy(
                op(FunctionToTest::SafeU256ToU128),
                arb_u256_larger_than(u128::MAX.into()).boxed(),
            ),
            Expect::AssertFailed,
        )
    }
}
