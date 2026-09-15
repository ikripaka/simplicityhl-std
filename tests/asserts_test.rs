mod common;

use rand::Rng;

use common::core::{Expect, run};

use simplicityhl_std::artifacts::asserts_test::AssertsTestProgram;
use simplicityhl_std::artifacts::asserts_test::derived_asserts_test::{
    AssertsTestArguments, AssertsTestWitness,
};

// Dispatch indices — must match the `if_test_this_function(N, ..)` arms in
// simf/asserts_test.simf.
enum FunctionToTest {
    AssertEq1,
    AssertEq8,
    AssertEq16,
    AssertEq32,
    AssertEq64,
    AssertEq128,
    AssertEq256,
    AssertEqBool,

    AssertNone1,
    AssertNone8,
    AssertNone16,
    AssertNone32,
    AssertNone64,
    AssertNone128,
    AssertNone256,
}

const DEFAULT_SOME_U8: Option<u8> = Some(0);
const DEFAULT_SOME_U16: Option<u16> = Some(0);
const DEFAULT_SOME_U32: Option<u32> = Some(0);
const DEFAULT_SOME_U64: Option<u64> = Some(0);
const DEFAULT_SOME_U128: Option<u128> = Some(0);
const DEFAULT_SOME_U256: Option<[u8; 32]> = Some([0; 32]);

fn program() -> AssertsTestProgram {
    AssertsTestProgram::new(AssertsTestArguments {})
}

/// Returns two values in `[min, max]` that are equal when `same`, distinct otherwise.
pub fn generate_uints_in_one_range(same: bool, min_val: u128, max_val: u128) -> (u128, u128) {
    let some_u = rand::thread_rng().gen_range(min_val..=max_val);

    if same {
        return (some_u, some_u);
    }

    assert!(
        min_val != max_val,
        "cannot generate distinct values in a single-value range"
    );

    let mut other_u = rand::thread_rng().gen_range(min_val..=max_val);

    while other_u == some_u {
        other_u = rand::thread_rng().gen_range(min_val..=max_val);
    }

    (some_u, other_u)
}

/// Builds the witness for one assert call. `same` controls the two `assert_eq`
/// args; `none` makes the single `assert_none` arg `None`.
fn build_witness(function: FunctionToTest, same: bool, none: bool) -> AssertsTestWitness {
    let mut witness = AssertsTestWitness {
        function_index: 0,
        first_arg_u1: DEFAULT_SOME_U8, // u1 in Simplicity is represented as u8
        second_arg_u1: DEFAULT_SOME_U8,
        first_arg_u8: DEFAULT_SOME_U8,
        second_arg_u8: DEFAULT_SOME_U8,
        first_arg_u16: DEFAULT_SOME_U16,
        second_arg_u16: DEFAULT_SOME_U16,
        first_arg_u32: DEFAULT_SOME_U32,
        second_arg_u32: DEFAULT_SOME_U32,
        first_arg_u64: DEFAULT_SOME_U64,
        second_arg_u64: DEFAULT_SOME_U64,
        first_arg_u128: DEFAULT_SOME_U128,
        second_arg_u128: DEFAULT_SOME_U128,
        first_arg_u256: DEFAULT_SOME_U256,
        second_arg_u256: DEFAULT_SOME_U256,
    };

    match function {
        FunctionToTest::AssertEq1 => {
            let (a, b) = generate_uints_in_one_range(same, 0, 1u128);
            (witness.first_arg_u1, witness.second_arg_u1) = (Some(a as u8), Some(b as u8));
        }
        FunctionToTest::AssertEq8 => {
            let (a, b) = generate_uints_in_one_range(same, 0, u8::MAX as u128);
            (witness.first_arg_u8, witness.second_arg_u8) = (Some(a as u8), Some(b as u8));
        }
        FunctionToTest::AssertEq16 => {
            let (a, b) = generate_uints_in_one_range(same, 0, u16::MAX as u128);
            (witness.first_arg_u16, witness.second_arg_u16) = (Some(a as u16), Some(b as u16));
        }
        FunctionToTest::AssertEq32 => {
            let (a, b) = generate_uints_in_one_range(same, 0, u32::MAX as u128);
            (witness.first_arg_u32, witness.second_arg_u32) = (Some(a as u32), Some(b as u32));
        }
        FunctionToTest::AssertEq64 => {
            let (a, b) = generate_uints_in_one_range(same, 0, u64::MAX as u128);
            (witness.first_arg_u64, witness.second_arg_u64) = (Some(a as u64), Some(b as u64));
        }
        FunctionToTest::AssertEq128 => {
            let (a, b) = generate_uints_in_one_range(same, 0, u128::MAX);
            (witness.first_arg_u128, witness.second_arg_u128) = (Some(a), Some(b));
        }
        FunctionToTest::AssertEq256 => {
            let (a, b) = generate_uints_in_one_range(same, 0, u8::MAX as u128);
            (witness.first_arg_u256, witness.second_arg_u256) =
                (Some([a as u8; 32]), Some([b as u8; 32]));
        }
        FunctionToTest::AssertEqBool => {
            let (a, b) = generate_uints_in_one_range(same, 0, 1u128);
            (witness.first_arg_u1, witness.second_arg_u1) = (Some(a as u8), Some(b as u8));
        }
        FunctionToTest::AssertNone1 => {
            if none {
                witness.first_arg_u1 = None;
            }
        }
        FunctionToTest::AssertNone8 => {
            if none {
                witness.first_arg_u8 = None;
            }
        }
        FunctionToTest::AssertNone16 => {
            if none {
                witness.first_arg_u16 = None;
            }
        }
        FunctionToTest::AssertNone32 => {
            if none {
                witness.first_arg_u32 = None;
            }
        }
        FunctionToTest::AssertNone64 => {
            if none {
                witness.first_arg_u64 = None;
            }
        }
        FunctionToTest::AssertNone128 => {
            if none {
                witness.first_arg_u128 = None;
            }
        }
        FunctionToTest::AssertNone256 => {
            if none {
                witness.first_arg_u256 = None;
            }
        }
    }

    witness.function_index = function as u8;
    witness
}

fn run_assert(
    context: &simplex::TestContext,
    function: FunctionToTest,
    same: bool,
    none: bool,
    expect: Expect,
) -> anyhow::Result<()> {
    run(
        context,
        program(),
        build_witness(function, same, none),
        expect,
    )
}

mod asserts_test {
    use super::*;

    // ---------- assert_eq: happy = equal args, unhappy = distinct args ----------
    #[simplex::test]
    fn assert_eq_1_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(&context, FunctionToTest::AssertEq1, true, false, Expect::Ok)
    }

    #[simplex::test]
    fn assert_eq_1_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq1,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_eq_8_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(&context, FunctionToTest::AssertEq8, true, false, Expect::Ok)
    }

    #[simplex::test]
    fn assert_eq_8_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq8,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_eq_16_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq16,
            true,
            false,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_eq_16_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq16,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_eq_32_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq32,
            true,
            false,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_eq_32_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq32,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_eq_64_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq64,
            true,
            false,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_eq_64_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq64,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_eq_128_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq128,
            true,
            false,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_eq_128_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq128,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_eq_256_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq256,
            true,
            false,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_eq_256_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEq256,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_eq_bool_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEqBool,
            true,
            false,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_eq_bool_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertEqBool,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    // ---------- assert_none: happy = None arg, unhappy = Some arg ----------
    #[simplex::test]
    fn assert_none_1_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone1,
            false,
            true,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_none_1_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone1,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_none_8_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone8,
            false,
            true,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_none_8_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone8,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_none_16_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone16,
            false,
            true,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_none_16_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone16,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_none_32_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone32,
            false,
            true,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_none_32_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone32,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_none_64_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone64,
            false,
            true,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_none_64_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone64,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_none_128_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone128,
            false,
            true,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_none_128_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone128,
            false,
            false,
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn assert_none_256_happy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone256,
            false,
            true,
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn assert_none_256_unhappy_path(context: simplex::TestContext) -> anyhow::Result<()> {
        run_assert(
            &context,
            FunctionToTest::AssertNone256,
            false,
            false,
            Expect::AssertFailed,
        )
    }
}

mod asserts_test_fuzz {
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

    type AssertsFuzzEngineBuilder =
        FuzzEngineBuilder<AssertsTestProgram, AssertsTestArguments, AssertsTestWitness>;

    fn initial_transaction() -> FinalTransaction {
        let mut tx = FinalTransaction::new();
        tx.add_input(PartialInput::new(UTXO::default()), RequiredSignature::None);
        tx
    }

    fn transaction_builder() -> Result<FinalTransactionBuilder, FuzzError> {
        FinalTransactionBuilder::new(initial_transaction(), [PROGRAM_TARGET])
    }

    fn arb_u1() -> impl Strategy<Value = u8> {
        any::<bool>().prop_map(u8::from)
    }

    fn arb_u8() -> impl Strategy<Value = u8> {
        any::<u8>()
    }

    fn arb_non_zero_u8() -> impl Strategy<Value = u8> {
        arb_u8().prop_filter("u8 should not be zero", |value| *value != 0)
    }

    fn arb_u16() -> impl Strategy<Value = u16> {
        any::<u16>()
    }

    fn arb_non_zero_u16() -> impl Strategy<Value = u16> {
        arb_u16().prop_filter("u16 should not be zero", |value| *value != 0)
    }

    fn arb_u32() -> impl Strategy<Value = u32> {
        any::<u32>()
    }

    fn arb_non_zero_u32() -> impl Strategy<Value = u32> {
        arb_u32().prop_filter("u32 should not be zero", |value| *value != 0)
    }

    fn arb_u64() -> impl Strategy<Value = u64> {
        any::<u64>()
    }

    fn arb_non_zero_u64() -> impl Strategy<Value = u64> {
        arb_u64().prop_filter("u64 should not be zero", |value| *value != 0)
    }

    fn arb_u128() -> impl Strategy<Value = u128> {
        any::<u128>()
    }

    fn arb_non_zero_u128() -> impl Strategy<Value = u128> {
        arb_u128().prop_filter("u128 should not be zero", |value| *value != 0)
    }

    fn arb_u256_be() -> impl Strategy<Value = [u8; 32]> {
        any::<[u8; 32]>()
    }

    fn default_witness(function_index: u8) -> AssertsTestWitness {
        AssertsTestWitness {
            function_index,
            first_arg_u1: DEFAULT_SOME_U8,
            second_arg_u1: DEFAULT_SOME_U8,
            first_arg_u8: DEFAULT_SOME_U8,
            second_arg_u8: DEFAULT_SOME_U8,
            first_arg_u16: DEFAULT_SOME_U16,
            second_arg_u16: DEFAULT_SOME_U16,
            first_arg_u32: DEFAULT_SOME_U32,
            second_arg_u32: DEFAULT_SOME_U32,
            first_arg_u64: DEFAULT_SOME_U64,
            second_arg_u64: DEFAULT_SOME_U64,
            first_arg_u128: DEFAULT_SOME_U128,
            second_arg_u128: DEFAULT_SOME_U128,
            first_arg_u256: DEFAULT_SOME_U256,
            second_arg_u256: DEFAULT_SOME_U256,
        }
    }

    fn assert_strategy(
        inputs: BoxedStrategy<AssertsTestWitness>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<AssertsTestArguments, AssertsTestWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(|witness| {
                let arguments: Arguments = AssertsTestArguments {}.into();
                let witness: WitnessValues = witness.into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_assert_fuzz(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
        expect: Expect,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("assert", expect));

        Ok(())
    }

    #[simplex::fuzz]
    fn assert_eq_1_happy_path(fuzz_engine_builder: AssertsFuzzEngineBuilder) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq1 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u1()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u1 = Some(value);
                        witness.second_arg_u1 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_1_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq1 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u1()
                    .prop_map(move |first| (first, 1 - first))
                    .prop_map(move |(first, second)| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u1 = Some(first);
                        witness.second_arg_u1 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_8_happy_path(fuzz_engine_builder: AssertsFuzzEngineBuilder) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq8 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u8()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u8 = Some(value);
                        witness.second_arg_u8 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_8_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq8 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_non_zero_u8()
                    .prop_map(move |first| (first, first - 1))
                    .prop_map(move |(first, second)| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u8 = Some(first);
                        witness.second_arg_u8 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_16_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq16 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u16()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u16 = Some(value);
                        witness.second_arg_u16 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_16_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq16 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_non_zero_u16()
                    .prop_map(move |first| (first, first - 1))
                    .prop_map(move |(first, second)| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u16 = Some(first);
                        witness.second_arg_u16 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_32_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq32 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u32()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u32 = Some(value);
                        witness.second_arg_u32 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_32_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq32 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_non_zero_u32()
                    .prop_map(move |first| (first, first - 1))
                    .prop_map(move |(first, second)| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u32 = Some(first);
                        witness.second_arg_u32 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_64_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq64 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u64()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u64 = Some(value);
                        witness.second_arg_u64 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_64_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq64 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_non_zero_u64()
                    .prop_map(move |first| (first, first - 1))
                    .prop_map(move |(first, second)| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u64 = Some(first);
                        witness.second_arg_u64 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_128_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq128 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u128()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u128 = Some(value);
                        witness.second_arg_u128 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_128_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq128 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_non_zero_u128()
                    .prop_map(move |first| (first, first - 1))
                    .prop_map(move |(first, second)| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u128 = Some(first);
                        witness.second_arg_u128 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_256_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq256 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u256_be()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u256 = Some(value);
                        witness.second_arg_u256 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_256_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEq256 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                (arb_u256_be(), arb_non_zero_u8())
                    .prop_map(|(first, difference)| {
                        let mut second = first;
                        second[0] ^= difference;
                        (first, second)
                    })
                    .prop_map(move |(first, second)| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u256 = Some(first);
                        witness.second_arg_u256 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_bool_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEqBool as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u1()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u1 = Some(value);
                        witness.second_arg_u1 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_eq_bool_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertEqBool as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u1()
                    .prop_map(move |first| (first, 1 - first))
                    .prop_map(move |(first, second)| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u1 = Some(first);
                        witness.second_arg_u1 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_none_1_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone1 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u1()
                    .prop_map(move |second| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u1 = None;
                        witness.second_arg_u1 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_none_1_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone1 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u1()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u1 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_none_8_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone8 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u8()
                    .prop_map(move |second| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u8 = None;
                        witness.second_arg_u8 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_none_8_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone8 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u8()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u8 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_none_16_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone16 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u16()
                    .prop_map(move |second| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u16 = None;
                        witness.second_arg_u16 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_none_16_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone16 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u16()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u16 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_none_32_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone32 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u32()
                    .prop_map(move |second| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u32 = None;
                        witness.second_arg_u32 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_none_32_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone32 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u32()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u32 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_none_64_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone64 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u64()
                    .prop_map(move |second| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u64 = None;
                        witness.second_arg_u64 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_none_64_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone64 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u64()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u64 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_none_128_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone128 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u128()
                    .prop_map(move |second| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u128 = None;
                        witness.second_arg_u128 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_none_128_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone128 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u128()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u128 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn assert_none_256_happy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone256 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u256_be()
                    .prop_map(move |second| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u256 = None;
                        witness.second_arg_u256 = Some(second);
                        witness
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn assert_none_256_unhappy_path(
        fuzz_engine_builder: AssertsFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        let function_index = FunctionToTest::AssertNone256 as u8;

        run_assert_fuzz(
            fuzz_engine_builder,
            assert_strategy(
                arb_u256_be()
                    .prop_map(move |value| {
                        let mut witness = default_witness(function_index);
                        witness.first_arg_u256 = Some(value);
                        witness
                    })
                    .boxed(),
            ),
            Expect::AssertFailed,
        )
    }
}
