mod common;

use primitive_types::U256;

use crate::common::helper::generate_u256;
use common::core::{Expect, run};

use simplicityhl_std::artifacts::u256_test_div::U256TestDivProgram;
use simplicityhl_std::artifacts::u256_test_div::derived_u256_test_div::{
    U256TestDivArguments, U256TestDivWitness,
};

enum FunctionToTest {
    DivMod256_64,
    DivMod256_128,
    DivMod256,
    Div256,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

const DEFAULT_EXPECTED: [u8; 32] = [0; 32];

fn program() -> U256TestDivProgram {
    U256TestDivProgram::new(U256TestDivArguments {})
}

fn build_witness(
    function: u8,
    a: [u8; 32],
    b: [u8; 32],
    expected: Option<[u8; 32]>,
    second_expected: [u8; 32],
) -> U256TestDivWitness {
    U256TestDivWitness {
        function_index: function,
        first_arg: a,
        second_arg: b,
        expected,
        second_expected,
    }
}

mod u256_tests_arithmetic {
    use super::*;

    #[simplex::test]
    fn test_div_mod_256_64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b = generate_u256(U256::one(), U256::from(u64::MAX));

        let q = (a / b).to_big_endian();
        let r = (a % b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256_64),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(q),
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn test_div_mod_256_64_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b = [0; 32];

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256_64),
                a.to_big_endian(),
                b,
                Some(DEFAULT_EXPECTED),
                DEFAULT_EXPECTED,
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn test_div_mod_256_128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b = generate_u256(U256::from(u64::MAX) + 1, U256::from(u128::MAX));

        let q = (a / b).to_big_endian();
        let r = (a % b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256_128),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(q),
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn test_div_mod_256_128_b_fits_into_u64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b = generate_u256(U256::one(), U256::from(u64::MAX));

        let q = (a / b).to_big_endian();
        let r = (a % b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256_128),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(q),
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn test_div_mod_256_128_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b = [0; 32];

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256_128),
                a.to_big_endian(),
                b,
                Some(DEFAULT_EXPECTED),
                DEFAULT_EXPECTED,
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn test_div_mod_256_128_a_eq_b(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = (generate_u256(U256::one(), U256::from(u128::MAX))).to_big_endian();

        let q = U256::one().to_big_endian();
        let r = U256::zero().to_big_endian();

        run(
            &context,
            program(),
            build_witness(op(FunctionToTest::DivMod256_128), a, a, Some(q), r),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_mod_256_a_less_than_b(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::one(), U256::MAX - 1);
        let b = generate_u256(a + 1, U256::MAX);

        let q = (a / b).to_big_endian();
        let r = (a % b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(q),
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_mod_256_div_128(context: simplex::TestContext) -> anyhow::Result<()> {
        let b = generate_u256(U256::one(), U256::from(u128::MAX));
        let a = generate_u256(b, U256::from(u128::MAX));

        let q = (a / b).to_big_endian();
        let r = (a % b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(q),
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_mod_256_q_is_1(context: simplex::TestContext) -> anyhow::Result<()> {
        // case where a >= b and a_high = b_high != 0
        let b_low = generate_u256(U256::zero(), U256::from(u128::MAX));
        let a_low = generate_u256(b_low, U256::from(u128::MAX));
        let high = generate_u256(U256::one(), U256::from(u128::MAX));

        let a = (high << 128) | (a_low);
        let b = (high << 128) | (b_low);

        let q = (a / b).to_big_endian();
        let r = (a % b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(q),
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_mod_256_b_fits_into_u128(context: simplex::TestContext) -> anyhow::Result<()> {
        let b = generate_u256(U256::one(), U256::from(u128::MAX));
        let a = generate_u256(U256::from(u128::MAX) + 1, U256::MAX);

        let q = (a / b).to_big_endian();
        let r = (a % b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(q),
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_mod_256_b_is_u256(context: simplex::TestContext) -> anyhow::Result<()> {
        let b = generate_u256(U256::one(), U256::MAX - 1);
        let a = generate_u256(b + 1, U256::MAX);

        let q = (a / b).to_big_endian();
        let r = (a % b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(q),
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_mod_256_a_equal_b(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::one(), U256::MAX).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256),
                a,
                a,
                Some(U256::one().to_big_endian()),
                [0; 32],
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_mod_256_equal_high_words_max_low_diff(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let high = generate_u256(U256::one(), U256::from(u128::MAX));

        let a = ((high << 128) | (U256::from(u128::MAX))).to_big_endian();
        let b = (high << 128).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256),
                a,
                b,
                Some(U256::one().to_big_endian()),
                U256::from(u128::MAX).to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_mod_256_eq_high_words_a_less_than_b(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let high = generate_u256(U256::one(), U256::from(u128::MAX));

        let a = (high << 128).to_big_endian();
        let b = ((high << 128) | (U256::from(u128::MAX))).to_big_endian();

        run(
            &context,
            program(),
            build_witness(op(FunctionToTest::DivMod256), a, b, Some([0; 32]), a),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_mod_256_edge_case(context: simplex::TestContext) -> anyhow::Result<()> {
        let a: U256 = U256::from(2).pow(U256::from(255));
        let b = U256::from(2).pow(U256::from(127)) + U256::from(2).pow(U256::from(64)) - 1;

        let (q, r) = a.div_mod(b);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(q.to_big_endian()),
                r.to_big_endian(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_256(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b = generate_u256(U256::one(), U256::MAX);
        let result = (a / b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Div256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result),
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_div_256_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b = [0; 32];

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Div256),
                a.to_big_endian(),
                b,
                Some(DEFAULT_EXPECTED),
                DEFAULT_EXPECTED,
            ),
            Expect::AssertFailed,
        )
    }
}

mod u256_tests_arithmetic_fuzz {
    use super::*;

    use common::core::{Expect, FuzzExecutionCheck};
    use simplex::fuzz;
    use simplex::fuzz::builders::{FinalTransactionBuilder, ProgramTarget};
    use simplex::fuzz::engine::FuzzStrategyBuilder;
    use simplex::fuzz::proptest::prelude::any;
    use simplex::fuzz::proptest::strategy::{BoxedStrategy, Strategy};
    use simplex::fuzz::{FuzzEngineBuilder, FuzzError};
    use simplex::simplicityhl::{Arguments, WitnessValues};
    use simplex::transaction::{FinalTransaction, PartialInput, RequiredSignature, UTXO};

    const PROGRAM_TARGET: ProgramTarget = ProgramTarget::Input(0);

    type U256DivFuzzEngineBuilder =
        FuzzEngineBuilder<U256TestDivProgram, U256TestDivArguments, U256TestDivWitness>;

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

    fn arb_non_zero_u256_be() -> impl Strategy<Value = U256> {
        arb_u256_be().prop_filter("u256 should not be zero", |value| !value.is_zero())
    }

    fn arb_u128() -> impl Strategy<Value = u128> {
        any::<u128>()
    }

    fn arb_non_zero_u128() -> impl Strategy<Value = u128> {
        arb_u128().prop_filter("u128 should not be zero", |value| *value != 0)
    }

    fn arb_128bit_u256_be() -> impl Strategy<Value = U256> {
        arb_u128().prop_map(U256::from)
    }

    fn arb_non_zero_128bit_u256_be() -> impl Strategy<Value = U256> {
        arb_non_zero_u128().prop_map(U256::from)
    }

    fn arb_127bit_u256_be() -> impl Strategy<Value = U256> {
        arb_u128().prop_map(|value| U256::from(value >> 1))
    }

    fn arb_non_zero_127bit_u256_be() -> impl Strategy<Value = U256> {
        arb_127bit_u256_be()
            .prop_filter("127-bit u256 should not be zero", |value| !value.is_zero())
    }

    fn arb_255bit_u256_be() -> impl Strategy<Value = U256> {
        arb_u256_be().prop_map(|x| x >> 1)
    }

    fn arb_non_zero_u64() -> impl Strategy<Value = u64> {
        any::<u64>().prop_filter("u64 should not be zero", |value| *value != 0)
    }

    fn arb_non_zero_64bit_u256_be() -> impl Strategy<Value = U256> {
        arb_non_zero_u64().prop_map(U256::from)
    }

    fn arb_128bit_u256_not_fitting_u64_be() -> impl Strategy<Value = U256> {
        arb_u128().prop_map(|value| U256::from(value | (1_u128 << 64)))
    }

    fn successful_division_strategy(
        function_index: u8,
        inputs: BoxedStrategy<(U256, U256)>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U256TestDivArguments, U256TestDivWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(a, b)| {
                let (quotient, remainder) = a.div_mod(b);
                let arguments: Arguments = U256TestDivArguments {}.into();
                let witness: WitnessValues = build_witness(
                    function_index,
                    a.to_big_endian(),
                    b.to_big_endian(),
                    Some(quotient.to_big_endian()),
                    remainder.to_big_endian(),
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn zero_divisor_strategy(
        function_index: u8,
        inputs: BoxedStrategy<U256>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U256TestDivArguments, U256TestDivWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |a| {
                let arguments: Arguments = U256TestDivArguments {}.into();
                let witness: WitnessValues = build_witness(
                    function_index,
                    a.to_big_endian(),
                    DEFAULT_EXPECTED,
                    Some(DEFAULT_EXPECTED),
                    DEFAULT_EXPECTED,
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_division_fuzz(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
        expect: Expect,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new("u256 division", expect));

        Ok(())
    }

    #[simplex::fuzz]
    fn test_div_mod_256_64(fuzz_engine_builder: U256DivFuzzEngineBuilder) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256_64),
                (arb_u256_be(), arb_non_zero_64bit_u256_be()).boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn test_div_mod_256_64_overflow(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            zero_divisor_strategy(op(FunctionToTest::DivMod256_64), arb_u256_be().boxed()),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn test_div_mod_256_128(fuzz_engine_builder: U256DivFuzzEngineBuilder) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256_128),
                (arb_u256_be(), arb_128bit_u256_not_fitting_u64_be()).boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn test_div_mod_256_128_b_fits_into_u64(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256_128),
                (arb_u256_be(), arb_non_zero_64bit_u256_be()).boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn test_div_mod_256_128_overflow(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            zero_divisor_strategy(op(FunctionToTest::DivMod256_128), arb_u256_be().boxed()),
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn test_div_mod_256_128_a_eq_b(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256_128),
                arb_non_zero_128bit_u256_be()
                    .prop_map(|value| (value, value))
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_test_div_mod_256_a_less_than_b(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256),
                arb_255bit_u256_be()
                    .prop_map(|a| (a, a + U256::one()))
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_test_div_mod_256_div_128(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256),
                (arb_non_zero_127bit_u256_be(), arb_127bit_u256_be())
                    .prop_map(|(b, offset)| (b + offset, b))
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_test_div_mod_256_q_is_1(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256),
                (
                    arb_non_zero_128bit_u256_be(),
                    arb_127bit_u256_be(),
                    arb_127bit_u256_be(),
                )
                    .prop_map(|(high, b_low, delta)| {
                        let high = high << 128;
                        let b = high | b_low;
                        let a = high | (b_low + delta);

                        (a, b)
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_test_div_mod_256_b_fits_into_u128(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256),
                (
                    arb_non_zero_128bit_u256_be(),
                    arb_128bit_u256_be(),
                    arb_non_zero_128bit_u256_be(),
                )
                    .prop_map(|(high, low, b)| ((high << 128) | low, b))
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_test_div_mod_256_b_is_u256(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256),
                (
                    arb_non_zero_127bit_u256_be(),
                    arb_128bit_u256_be(),
                    arb_non_zero_128bit_u256_be(),
                )
                    .prop_map(|(high, low, offset)| {
                        let b = (high << 128) | low;

                        (b + offset, b)
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_test_div_mod_256_a_equal_b(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256),
                arb_non_zero_u256_be()
                    .prop_map(|value| (value, value))
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_test_div_mod_256_equal_high_words_max_low_diff(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256),
                arb_non_zero_128bit_u256_be()
                    .prop_map(|high| {
                        let b = high << 128;

                        (b | U256::from(u128::MAX), b)
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_test_div_mod_256_eq_high_words_a_less_than_b(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::DivMod256),
                arb_non_zero_128bit_u256_be()
                    .prop_map(|high| {
                        let a = high << 128;

                        (a, a | U256::from(u128::MAX))
                    })
                    .boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_test_div_256(fuzz_engine_builder: U256DivFuzzEngineBuilder) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            successful_division_strategy(
                op(FunctionToTest::Div256),
                (arb_u256_be(), arb_non_zero_u256_be()).boxed(),
            ),
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn u256_test_div_256_overflow(
        fuzz_engine_builder: U256DivFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_division_fuzz(
            fuzz_engine_builder,
            zero_divisor_strategy(op(FunctionToTest::Div256), arb_u256_be().boxed()),
            Expect::AssertFailed,
        )
    }
}
