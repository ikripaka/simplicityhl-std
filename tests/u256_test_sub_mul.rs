mod common;

use primitive_types::U256;
use rand::Rng;

use crate::common::helper::{DEFAULT_BOOL, generate_u256};
use common::core::{Expect, run};

use simplicityhl_std::artifacts::u256_test_sub_mul::U256TestSubMulProgram;
use simplicityhl_std::artifacts::u256_test_sub_mul::derived_u256_test_sub_mul::{
    U256TestSubMulArguments, U256TestSubMulWitness,
};

enum FunctionToTest {
    Sub256,
    Mul256,
    Mul256_64,
    Mul256_128,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

const DEFAULT_EXPECTED: [u8; 32] = [0; 32];

fn program() -> U256TestSubMulProgram {
    U256TestSubMulProgram::new(U256TestSubMulArguments {})
}

fn build_witness(
    function: u8,
    a: [u8; 32],
    b: [u8; 32],
    expected: Option<[u8; 32]>,
    expected_bool: bool,
    second_expected: [u8; 32],
) -> U256TestSubMulWitness {
    U256TestSubMulWitness {
        function_index: function,
        first_arg: a,
        second_arg: b,
        expected,
        expected_bool,
        second_expected,
    }
}

fn split_u512(a: [u8; 64]) -> ([u8; 32], [u8; 32]) {
    let high = U256::from_big_endian(&a[0..32]);
    let low = U256::from_big_endian(&a[32..64]);

    (high.to_big_endian(), low.to_big_endian())
}

mod u256_tests_arithmetic {
    use super::*;

    #[simplex::test]
    fn u256_test_sub_256_not_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b = generate_u256(U256::zero(), a);
        let result = (a - b).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_sub_256_a_eq_b(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX).to_big_endian();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub256),
                a,
                a,
                Some([0; 32]),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_sub_256_a_low_eq_b_low(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::zero(), U256::MAX);
        let b_high = rand::thread_rng().gen_range(0..=u128::MAX);

        let low: u128 = a.low_u128();
        let b = (U256::from(b_high) << 128) | U256::from(low);

        let (result, carry) = a.overflowing_sub(b);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result.to_big_endian()),
                carry,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_sub_256_diff_is_u128_max(context: simplex::TestContext) -> anyhow::Result<()> {
        let a_low: u128 = u128::MAX;

        let a_high = rand::thread_rng().gen_range(0..=u128::MAX);
        let b_high = rand::thread_rng().gen_range(0..=u128::MAX);

        let a = (U256::from(a_high) << 128) | U256::from(a_low);
        let b = (U256::from(b_high)) << 128; // b_low is 0

        let (result, carry) = a.overflowing_sub(b);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result.to_big_endian()),
                carry,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_sub_256_diff_is_u256_max(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = U256::MAX.to_big_endian();
        let b = U256::zero();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub256),
                a,
                b.to_big_endian(),
                Some(a),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_sub_256_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::one(), U256::MAX - 1);
        let b = U256::MAX;
        let result = a + 1;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result.to_big_endian()),
                true,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_mul_256(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::one(), U256::MAX);
        let b = generate_u256(U256::one(), U256::MAX);
        let result = a.full_mul(b).to_big_endian();

        let (result_high, result_low) = split_u512(result);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Mul256),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result_high),
                DEFAULT_BOOL,
                result_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_mul_256_64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::one(), U256::MAX);
        let b = generate_u256(U256::one(), U256::from(u64::MAX));
        let result = a.full_mul(b).to_big_endian();

        let (result_high, result_low) = split_u512(result);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Mul256_64),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result_high),
                DEFAULT_BOOL,
                result_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u256_test_mul_256_128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = generate_u256(U256::one(), U256::MAX);
        let b = generate_u256(U256::one(), U256::from(u128::MAX));
        let result = a.full_mul(b).to_big_endian();

        let (result_high, result_low) = split_u512(result);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Mul256_128),
                a.to_big_endian(),
                b.to_big_endian(),
                Some(result_high),
                DEFAULT_BOOL,
                result_low,
            ),
            Expect::Ok,
        )
    }
}

mod u256_tests_arithmetic_fuzz {
    use super::*;

    use common::core::{Expect, FuzzExecutionCheck};
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

    type U256SubMulFuzzEngineBuilder =
        FuzzEngineBuilder<U256TestSubMulProgram, U256TestSubMulArguments, U256TestSubMulWitness>;

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

    fn arb_u128() -> impl Strategy<Value = u128> {
        any::<u128>()
    }

    fn arb_128bit_u256_be() -> impl Strategy<Value = U256> {
        arb_u128().prop_map(U256::from)
    }

    fn arb_u64() -> impl Strategy<Value = u64> {
        any::<u64>()
    }

    fn arb_64bit_u256_be() -> impl Strategy<Value = U256> {
        arb_u64().prop_map(U256::from)
    }

    fn arb_255bit_u256_be() -> impl Strategy<Value = U256> {
        arb_u256_be().prop_map(|value| value >> 1)
    }

    fn subtraction_strategy(
        expected_bool: bool,
        inputs: BoxedStrategy<(U256, U256)>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U256TestSubMulArguments, U256TestSubMulWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(a, b)| {
                let (expected, _) = a.overflowing_sub(b);
                let arguments: Arguments = U256TestSubMulArguments {}.into();
                let witness: WitnessValues = build_witness(
                    op(FunctionToTest::Sub256),
                    a.to_big_endian(),
                    b.to_big_endian(),
                    Some(expected.to_big_endian()),
                    expected_bool,
                    DEFAULT_EXPECTED,
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn multiplication_strategy(
        function_index: u8,
        expected_bool: bool,
        inputs: BoxedStrategy<(U256, U256)>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U256TestSubMulArguments, U256TestSubMulWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(move |(a, b)| {
                let (high, low) = split_u512(a.full_mul(b).to_big_endian());
                let arguments: Arguments = U256TestSubMulArguments {}.into();
                let witness: WitnessValues = build_witness(
                    function_index,
                    a.to_big_endian(),
                    b.to_big_endian(),
                    Some(high),
                    expected_bool,
                    low,
                )
                .into();

                (arguments, witness)
            }))
            .build()
    }

    fn run_successful_fuzz(
        fuzz_engine_builder: U256SubMulFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new(
                "u256 subtraction or multiplication",
                Expect::Ok,
            ));

        Ok(())
    }

    #[simplex::fuzz]
    fn u256_test_sub_256_not_overflow(
        fuzz_engine_builder: U256SubMulFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            subtraction_strategy(
                EXPECTED_FALSE,
                (arb_u256_be(), arb_u256_be())
                    .prop_map(|(a, mask)| (a, a & mask))
                    .boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_sub_256_a_eq_b(
        fuzz_engine_builder: U256SubMulFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            subtraction_strategy(
                EXPECTED_FALSE,
                arb_u256_be().prop_map(|value| (value, value)).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_sub_256_a_low_eq_b_low(
        fuzz_engine_builder: U256SubMulFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            subtraction_strategy(
                EXPECTED_FALSE,
                (arb_u256_be(), arb_128bit_u256_be())
                    .prop_map(|(a, b_high)| {
                        let b = ((a >> 128) & b_high) << 128 | U256::from(a.low_u128());

                        (a, b)
                    })
                    .boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_sub_256_diff_is_u128_max(
        fuzz_engine_builder: U256SubMulFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            subtraction_strategy(
                EXPECTED_FALSE,
                arb_128bit_u256_be()
                    .prop_map(|high| {
                        let a = (high << 128) | U256::from(u128::MAX);
                        let b = high << 128;

                        (a, b)
                    })
                    .boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_sub_256_diff_is_u256_max(
        fuzz_engine_builder: U256SubMulFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            subtraction_strategy(EXPECTED_FALSE, Just((U256::MAX, U256::zero())).boxed()),
        )
    }

    #[simplex::fuzz]
    fn u256_test_sub_256_overflow(
        fuzz_engine_builder: U256SubMulFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            subtraction_strategy(
                EXPECTED_TRUE,
                arb_255bit_u256_be().prop_map(|a| (a, U256::MAX)).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_mul_256(fuzz_engine_builder: U256SubMulFuzzEngineBuilder) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            multiplication_strategy(
                op(FunctionToTest::Mul256),
                EXPECTED_FALSE,
                (arb_u256_be(), arb_u256_be()).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_mul_256_64(
        fuzz_engine_builder: U256SubMulFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            multiplication_strategy(
                op(FunctionToTest::Mul256_64),
                EXPECTED_FALSE,
                (arb_u256_be(), arb_64bit_u256_be()).boxed(),
            ),
        )
    }

    #[simplex::fuzz]
    fn u256_test_mul_256_128(
        fuzz_engine_builder: U256SubMulFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_successful_fuzz(
            fuzz_engine_builder,
            multiplication_strategy(
                op(FunctionToTest::Mul256_128),
                EXPECTED_FALSE,
                (arb_u256_be(), arb_128bit_u256_be()).boxed(),
            ),
        )
    }
}
