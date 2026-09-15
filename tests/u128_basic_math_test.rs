mod common;

use primitive_types::U256;
use rand::Rng;

use crate::common::helper::DEFAULT_BOOL;
use common::core::{Expect, run};

use simplicityhl_std::artifacts::u128_basic_math_test::U128BasicMathTestProgram;
use simplicityhl_std::artifacts::u128_basic_math_test::derived_u128_basic_math_test::{
    U128BasicMathTestArguments, U128BasicMathTestWitness,
};

enum FunctionToTest {
    Add128,
    Add128_64,
    FullAdd128,
    Sub128,
    FullSub128,
    Mul128,
    Mul128_64,
    CalculateNormalizerBase64,
    EstimateQuotientDigitBase64,
    DivMod128_64,
    DivMod128,
    Div128,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

const DEFAULT_EXPECTED: u128 = 0;

fn program() -> U128BasicMathTestProgram {
    U128BasicMathTestProgram::new(U128BasicMathTestArguments {})
}

fn build_witness(
    function: u8,
    a: u128,
    b: u128,
    expected: Option<u128>,
    expected_bool: bool,
    second_expected: u128,
) -> U128BasicMathTestWitness {
    U128BasicMathTestWitness {
        function_index: function,
        first_arg: a,
        second_arg: b,
        expected,
        expected_bool,
        second_expected,
    }
}

fn split_helper(a: U256) -> (u128, u128) {
    let a_high = (a >> 128).as_u128();
    let a_low = a.low_u128();

    (a_high, a_low)
}

mod u128_tests_arithmetic {
    use super::*;

    #[simplex::test]
    fn u128_test_add_128_not_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX / 2);
        let b = rand::thread_rng().gen_range(0..=u128::MAX / 2);
        let result = a + b;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Add128),
                a,
                b,
                Some(result),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_add_128_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = u128::MAX;
        let b = rand::thread_rng().gen_range(1..=u128::MAX);
        let result = b - 1;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Add128),
                a,
                b,
                Some(result),
                true,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_add_128_64_not_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX / 2);
        let b = rand::thread_rng().gen_range(0..=u64::MAX) as u128;
        let result = a + b;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Add128_64),
                a,
                b,
                Some(result),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_add_128_64_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = u128::MAX;
        let b = rand::thread_rng().gen_range(1..=u64::MAX) as u128;
        let result = b - 1;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Add128_64),
                a,
                b,
                Some(result),
                true,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_full_add_128_not_overflow_carry_low_false(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX / 2);
        let b = rand::thread_rng().gen_range(0..=u128::MAX / 2);
        let result = a + b;
        let result_carry = false;
        let carry_low = 0_u128;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FullAdd128),
                a,
                b,
                Some(result),
                result_carry,
                carry_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_full_add_128_overflow_carry_low_false(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = u128::MAX;
        let b = rand::thread_rng().gen_range(1..=u128::MAX);
        let result = b - 1;
        let result_carry = true;
        let carry_low = 0_u128;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FullAdd128),
                a,
                b,
                Some(result),
                result_carry,
                carry_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_full_add_128_not_overflow_carry_low_true(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX / 2);
        let b = rand::thread_rng().gen_range(0..=u128::MAX / 2);
        let result = a + b + 1;
        let result_carry = false;
        let carry_low = 1_u128;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FullAdd128),
                a,
                b,
                Some(result),
                result_carry,
                carry_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_full_add_128_overflow_carry_low_true(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = u128::MAX;
        let b = rand::thread_rng().gen_range(1..=u128::MAX);
        let result = b;
        let result_carry = true;
        let carry_low = 1_u128;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FullAdd128),
                a,
                b,
                Some(result),
                result_carry,
                carry_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_sub_128_not_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);
        let b = rand::thread_rng().gen_range(0..=a);
        let result = a - b;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub128),
                a,
                b,
                Some(result),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_sub_128_a_eq_b(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub128),
                a,
                a,
                Some(0),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_sub_128_a_low_eq_b_low(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);
        let b_high = rand::thread_rng().gen_range(0..=u64::MAX);

        let low: u64 = a as u64;
        let b = ((b_high as u128) << 64) | (low as u128);

        let carry = a < b;
        let result = a.wrapping_sub(b);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub128),
                a,
                b,
                Some(result),
                carry,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_sub_128_diff_is_u64_max(context: simplex::TestContext) -> anyhow::Result<()> {
        let a_low: u64 = u64::MAX;

        let a_high = rand::thread_rng().gen_range(0..=u64::MAX);
        let b_high = rand::thread_rng().gen_range(0..=u64::MAX);

        let a = ((a_high as u128) << 64) | (a_low as u128);
        let b = (b_high as u128) << 64; // b_low is 0

        let carry = a < b;
        let result = a.wrapping_sub(b);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub128),
                a,
                b,
                Some(result),
                carry,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_sub_128_diff_is_u128_max(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = u128::MAX;
        let b = 0;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub128),
                a,
                b,
                Some(a),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_sub_128_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..u128::MAX);
        let b = u128::MAX;
        let result = a + 1;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Sub128),
                a,
                b,
                Some(result),
                true,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_full_sub_128_borrow_low_false(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);
        let b = rand::thread_rng().gen_range(0..=a);
        let result = a - b;
        let result_borrow = false;
        let borrow_low = 0_u128;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FullSub128),
                a,
                b,
                Some(result),
                result_borrow,
                borrow_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_full_sub_128_overflow_borrow_low_false(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..u128::MAX);
        let b = u128::MAX;
        let result = a + 1;
        let result_borrow = true;
        let borrow_low = 0_u128;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FullSub128),
                a,
                b,
                Some(result),
                result_borrow,
                borrow_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_full_sub_128_borrow_low_true(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);
        let b = rand::thread_rng().gen_range(0..a);
        let result = a - b - 1;
        let result_borrow = false;
        let borrow_low = 1_u128;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FullSub128),
                a,
                b,
                Some(result),
                result_borrow,
                borrow_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_full_sub_128_overflow_borrow_low_true(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..u128::MAX);
        let b = u128::MAX;
        let (result, result_borrow) = a.overflowing_sub(b);

        let borrow_low = 1_u128;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FullSub128),
                a,
                b,
                Some(result - 1),
                result_borrow,
                borrow_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_mul_128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..u128::MAX);
        let b = rand::thread_rng().gen_range(0..u128::MAX);
        let result = U256::from(a) * U256::from(b);

        let (result_high, result_low) = split_helper(result);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Mul128),
                a,
                b,
                Some(result_high),
                DEFAULT_BOOL,
                result_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_mul_128_64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..u128::MAX);
        let b = rand::thread_rng().gen_range(0..u64::MAX);
        let result = U256::from(a) * U256::from(b);

        let (result_high, result_low) = split_helper(result);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Mul128_64),
                a,
                b as u128,
                Some(result_high),
                DEFAULT_BOOL,
                result_low,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_calculate_normalizer_base_64_b_is_u64(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let threshold = 1u128 << 63;

        let b = rand::thread_rng().gen_range(1..threshold);

        let norm: u128 = threshold.div_ceil(b);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::CalculateNormalizerBase64),
                DEFAULT_EXPECTED,
                b,
                Some(norm),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_calculate_normalizer_base_64_b_is_big_enough_not_normalize(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let threshold = 1u128 << 63;

        let b = rand::thread_rng().gen_range(threshold..=u64::MAX as u128);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::CalculateNormalizerBase64),
                DEFAULT_EXPECTED,
                b,
                Some(1),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_calculate_normalizer_base_64_b_is_u128(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let threshold = 1u128 << 63;

        let b = rand::thread_rng().gen_range((u64::MAX as u128) + 1..=u128::MAX);
        let b_high = b >> 64;

        let norm: u128 = threshold.div_ceil(b_high);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::CalculateNormalizerBase64),
                DEFAULT_EXPECTED,
                b,
                Some(norm),
                true,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_calculate_normalizer_base_64_b_is_u64_fail(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let b = rand::thread_rng().gen_range((u64::MAX as u128) + 1..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::CalculateNormalizerBase64),
                DEFAULT_EXPECTED,
                b,
                Some(DEFAULT_EXPECTED),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u128_test_calculate_normalizer_base_64_b_is_u128_fail(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let b = rand::thread_rng().gen_range(1..=u64::MAX as u128);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::CalculateNormalizerBase64),
                DEFAULT_EXPECTED,
                b,
                Some(DEFAULT_EXPECTED),
                true,
                DEFAULT_EXPECTED,
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u128_test_calculate_normalizer_base_64_b_is_zero_fail(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let b = 0;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::CalculateNormalizerBase64),
                DEFAULT_EXPECTED,
                b,
                Some(DEFAULT_EXPECTED),
                false,
                DEFAULT_EXPECTED,
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u128_test_estimate_quotient_digit_base_64(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let threshold = 1u64 << 63;

        let b_high = rand::thread_rng().gen_range(threshold..=u64::MAX);
        let b_low = rand::thread_rng().gen_range(0..=u64::MAX);

        let a_high = rand::thread_rng().gen_range(0..b_high);
        let a_low = rand::thread_rng().gen_range(0..=u128::MAX);

        let a = ((U256::from(a_high)) << 128) | (U256::from(a_low));
        let b = ((b_high as u128) << 64) | (b_low as u128);

        let q = (a / b).as_u128();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::EstimateQuotientDigitBase64),
                a_high as u128,
                a_low,
                Some(q),
                DEFAULT_BOOL,
                b,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_estimate_quotient_digit_base_64_fail(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        // expected to fail because a is to big for q to fit unto u64
        let threshold = 1u64 << 63;

        let b_high = rand::thread_rng().gen_range(threshold..u64::MAX);
        let b_low = rand::thread_rng().gen_range(0..=u64::MAX);

        let a_high = rand::thread_rng().gen_range(b_high + 1..=u64::MAX);
        let a_low = rand::thread_rng().gen_range(0..=u128::MAX);

        let a = ((U256::from(a_high)) << 128) | (U256::from(a_low));
        let b = ((b_high as u128) << 64) | (b_low as u128);

        let q = (a / b).as_u128();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::EstimateQuotientDigitBase64),
                a_high as u128,
                a_low,
                Some(q),
                DEFAULT_BOOL,
                b,
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn test_div_mod_128_64(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);
        let b = rand::thread_rng().gen_range(1..=u64::MAX as u128);

        let q = a / b;
        let r = a % b;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod128_64),
                a,
                b,
                Some(q),
                DEFAULT_BOOL,
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn test_div_mod_128_64_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);
        let b = 0;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod128_64),
                a,
                b,
                Some(DEFAULT_EXPECTED),
                DEFAULT_BOOL,
                DEFAULT_EXPECTED,
            ),
            Expect::AssertFailed,
        )
    }

    #[simplex::test]
    fn u128_test_div_mod_128_a_less_than_b(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..u128::MAX);
        let b = rand::thread_rng().gen_range(a + 1..=u128::MAX);

        let q = a / b;
        let r = a % b;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod128),
                a,
                b,
                Some(q),
                DEFAULT_BOOL,
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_div_mod_128_div_64(context: simplex::TestContext) -> anyhow::Result<()> {
        let b = rand::thread_rng().gen_range(1..=u64::MAX);
        let a = rand::thread_rng().gen_range(b..=u64::MAX) as u128;

        let q = a / b as u128;
        let r = a % b as u128;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod128),
                a,
                b as u128,
                Some(q),
                DEFAULT_BOOL,
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_div_mod_128_q_is_1(context: simplex::TestContext) -> anyhow::Result<()> {
        // case where a >= b and a_high = b_high != 0
        let b_low = rand::thread_rng().gen_range(0..=u64::MAX);
        let a_low = rand::thread_rng().gen_range(b_low..=u64::MAX);
        let high = rand::thread_rng().gen_range(1..u64::MAX);

        let a = ((high as u128) << 64) | (a_low as u128);
        let b = ((high as u128) << 64) | (b_low as u128);

        let q = a / b;
        let r = a % b;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod128),
                a,
                b,
                Some(q),
                DEFAULT_BOOL,
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_div_mod_128_b_is_u64(context: simplex::TestContext) -> anyhow::Result<()> {
        let b = rand::thread_rng().gen_range(1..=u64::MAX as u128);
        let a = rand::thread_rng().gen_range(u64::MAX as u128 + 1..=u128::MAX);

        let q = a / b;
        let r = a % b;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod128),
                a,
                b,
                Some(q),
                DEFAULT_BOOL,
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_div_mod_128_b_is_u128(context: simplex::TestContext) -> anyhow::Result<()> {
        let b_high = rand::thread_rng().gen_range(1..u64::MAX);
        let a_high = rand::thread_rng().gen_range(b_high + 1..=u64::MAX);

        let a = ((a_high as u128) << 64) | (rand::thread_rng().gen_range(0..u64::MAX) as u128);
        let b = ((b_high as u128) << 64) | (rand::thread_rng().gen_range(0..u64::MAX) as u128);

        let q = a / b;
        let r = a % b;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod128),
                a,
                b,
                Some(q),
                DEFAULT_BOOL,
                r,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_div_mod_128_a_equal_b(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(1..=u128::MAX);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod128),
                a,
                a,
                Some(1u128),
                DEFAULT_BOOL,
                0u128,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_div_mod_128_equal_high_words_max_low_diff(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let high = rand::thread_rng().gen_range(1..=u64::MAX);

        let a = ((high as u128) << 64) | (u64::MAX as u128);
        let b = (high as u128) << 64;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod128),
                a,
                b,
                Some(1u128),
                DEFAULT_BOOL,
                u64::MAX as u128,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_div_mod_128_eq_high_words_a_less_than_b(
        context: simplex::TestContext,
    ) -> anyhow::Result<()> {
        let high = rand::thread_rng().gen_range(1..=u64::MAX);

        let a = (high as u128) << 64;
        let b = ((high as u128) << 64) | (u64::MAX as u128);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::DivMod128),
                a,
                b,
                Some(0u128),
                DEFAULT_BOOL,
                a,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_div_128(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);
        let b = rand::thread_rng().gen_range(1..=u128::MAX);
        let result = a / b;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Div128),
                a,
                b,
                Some(result),
                DEFAULT_BOOL,
                DEFAULT_EXPECTED,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn u128_test_div_128_overflow(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = rand::thread_rng().gen_range(0..=u128::MAX);
        let b = 0;

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::Div128),
                a,
                b,
                Some(DEFAULT_EXPECTED),
                DEFAULT_BOOL,
                DEFAULT_EXPECTED,
            ),
            Expect::AssertFailed,
        )
    }
}

mod u128_tests_arithmetic_fuzz {
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
    const EXPECTED_FALSE: bool = false;
    const EXPECTED_TRUE: bool = true;
    const CARRY_OR_BORROW_LOW_FALSE: u128 = 0;
    const CARRY_OR_BORROW_LOW_TRUE: u128 = 1;
    const NORMALIZER_THRESHOLD: u128 = 1 << 63;

    type U128BasicMathFuzzEngineBuilder = FuzzEngineBuilder<
        U128BasicMathTestProgram,
        U128BasicMathTestArguments,
        U128BasicMathTestWitness,
    >;
    type U128BasicMathInputs = (u8, u128, u128, Option<u128>, bool, u128);

    fn initial_transaction() -> FinalTransaction {
        let mut tx = FinalTransaction::new();
        tx.add_input(PartialInput::new(UTXO::default()), RequiredSignature::None);
        tx
    }

    fn transaction_builder() -> Result<FinalTransactionBuilder, FuzzError> {
        FinalTransactionBuilder::new(initial_transaction(), [PROGRAM_TARGET])
    }

    fn arb_non_zero_u64() -> impl Strategy<Value = u64> {
        any::<u64>().prop_filter("u64 should not be zero", |value| *value != 0)
    }

    fn arb_non_zero_u128() -> impl Strategy<Value = u128> {
        any::<u128>().prop_filter("u128 should not be zero", |value| *value != 0)
    }

    fn fuzz_strategy(
        inputs: BoxedStrategy<U128BasicMathInputs>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<U128BasicMathTestArguments, U128BasicMathTestWitness, _>::new()
            .with_custom_strategy(inputs.prop_map(
                |(function, a, b, expected, expected_bool, second_expected)| {
                    let arguments: Arguments = U128BasicMathTestArguments {}.into();
                    let witness: WitnessValues =
                        build_witness(function, a, b, expected, expected_bool, second_expected)
                            .into();

                    (arguments, witness)
                },
            ))
            .build()
    }

    fn run_u128_basic_math_fuzz(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
        strategy: BoxedStrategy<(Arguments, WitnessValues)>,
        test_name: &'static str,
        expect: Expect,
    ) -> anyhow::Result<()> {
        let transaction_builder = transaction_builder()?;

        fuzz_engine_builder
            .build(strategy, transaction_builder)
            .run_with_check(FuzzExecutionCheck::new(test_name, expect));

        Ok(())
    }

    fn full_add_reference(a: u128, b: u128, carry_low: bool) -> (u128, bool) {
        let (sum, carry) = a.overflowing_add(b);

        if carry_low {
            let (sum, carry_from_low) = sum.overflowing_add(1);
            (sum, carry || carry_from_low)
        } else {
            (sum, carry)
        }
    }

    fn full_sub_reference(a: u128, b: u128, borrow_low: bool) -> (u128, bool) {
        let (difference, borrow) = a.overflowing_sub(b);

        if borrow_low {
            let (difference, borrow_from_low) = difference.overflowing_sub(1);
            (difference, borrow || borrow_from_low)
        } else {
            (difference, borrow)
        }
    }

    #[simplex::fuzz]
    fn add_128(fuzz_engine_builder: U128BasicMathFuzzEngineBuilder) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), any::<u128>())
                    .prop_map(|(a, b)| {
                        let (result, carry) = a.overflowing_add(b);
                        (
                            op(FunctionToTest::Add128),
                            a,
                            b,
                            Some(result),
                            carry,
                            DEFAULT_EXPECTED,
                        )
                    })
                    .boxed(),
            ),
            "add_128",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn add_128_64(fuzz_engine_builder: U128BasicMathFuzzEngineBuilder) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), any::<u64>())
                    .prop_map(|(a, b)| {
                        let b = u128::from(b);
                        let (result, carry) = a.overflowing_add(b);
                        (
                            op(FunctionToTest::Add128_64),
                            a,
                            b,
                            Some(result),
                            carry,
                            DEFAULT_EXPECTED,
                        )
                    })
                    .boxed(),
            ),
            "add_128_64",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn full_add_128_without_carry_low(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), any::<u128>())
                    .prop_map(|(a, b)| {
                        let (result, carry) = full_add_reference(a, b, EXPECTED_FALSE);
                        (
                            op(FunctionToTest::FullAdd128),
                            a,
                            b,
                            Some(result),
                            carry,
                            CARRY_OR_BORROW_LOW_FALSE,
                        )
                    })
                    .boxed(),
            ),
            "full_add_128 without carry low",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn full_add_128_with_carry_low(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), any::<u128>())
                    .prop_map(|(a, b)| {
                        let (result, carry) = full_add_reference(a, b, EXPECTED_TRUE);
                        (
                            op(FunctionToTest::FullAdd128),
                            a,
                            b,
                            Some(result),
                            carry,
                            CARRY_OR_BORROW_LOW_TRUE,
                        )
                    })
                    .boxed(),
            ),
            "full_add_128 with carry low",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn sub_128(fuzz_engine_builder: U128BasicMathFuzzEngineBuilder) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), any::<u128>())
                    .prop_map(|(a, b)| {
                        let (result, borrow) = a.overflowing_sub(b);
                        (
                            op(FunctionToTest::Sub128),
                            a,
                            b,
                            Some(result),
                            borrow,
                            DEFAULT_EXPECTED,
                        )
                    })
                    .boxed(),
            ),
            "sub_128",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn full_sub_128_without_borrow_low(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), any::<u128>())
                    .prop_map(|(a, b)| {
                        let (result, borrow) = full_sub_reference(a, b, EXPECTED_FALSE);
                        (
                            op(FunctionToTest::FullSub128),
                            a,
                            b,
                            Some(result),
                            borrow,
                            CARRY_OR_BORROW_LOW_FALSE,
                        )
                    })
                    .boxed(),
            ),
            "full_sub_128 without borrow low",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn full_sub_128_with_borrow_low(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), any::<u128>())
                    .prop_map(|(a, b)| {
                        let (result, borrow) = full_sub_reference(a, b, EXPECTED_TRUE);
                        (
                            op(FunctionToTest::FullSub128),
                            a,
                            b,
                            Some(result),
                            borrow,
                            CARRY_OR_BORROW_LOW_TRUE,
                        )
                    })
                    .boxed(),
            ),
            "full_sub_128 with borrow low",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn mul_128(fuzz_engine_builder: U128BasicMathFuzzEngineBuilder) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), any::<u128>())
                    .prop_map(|(a, b)| {
                        let (high, low) = split_helper(U256::from(a) * U256::from(b));
                        (
                            op(FunctionToTest::Mul128),
                            a,
                            b,
                            Some(high),
                            EXPECTED_FALSE,
                            low,
                        )
                    })
                    .boxed(),
            ),
            "mul_128",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn mul_128_64(fuzz_engine_builder: U128BasicMathFuzzEngineBuilder) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), any::<u64>())
                    .prop_map(|(a, b)| {
                        let b = u128::from(b);
                        let (high, low) = split_helper(U256::from(a) * U256::from(b));
                        (
                            op(FunctionToTest::Mul128_64),
                            a,
                            b,
                            Some(high),
                            EXPECTED_FALSE,
                            low,
                        )
                    })
                    .boxed(),
            ),
            "mul_128_64",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn calculate_normalizer_base_64_for_u64(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                arb_non_zero_u64()
                    .prop_map(|b| {
                        let b = u128::from(b);
                        (
                            op(FunctionToTest::CalculateNormalizerBase64),
                            DEFAULT_EXPECTED,
                            b,
                            Some(NORMALIZER_THRESHOLD.div_ceil(b)),
                            EXPECTED_FALSE,
                            DEFAULT_EXPECTED,
                        )
                    })
                    .boxed(),
            ),
            "calculate_normalizer_base_64 for u64",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn calculate_normalizer_base_64_for_u128(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (arb_non_zero_u64(), any::<u64>())
                    .prop_map(|(high, low)| {
                        let b = (u128::from(high) << 64) | u128::from(low);
                        (
                            op(FunctionToTest::CalculateNormalizerBase64),
                            DEFAULT_EXPECTED,
                            b,
                            Some(NORMALIZER_THRESHOLD.div_ceil(u128::from(high))),
                            EXPECTED_TRUE,
                            DEFAULT_EXPECTED,
                        )
                    })
                    .boxed(),
            ),
            "calculate_normalizer_base_64 for u128",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn calculate_normalizer_base_64_rejects_wrong_u64_flag(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (arb_non_zero_u64(), any::<u64>())
                    .prop_map(|(high, low)| {
                        let b = (u128::from(high) << 64) | u128::from(low);
                        (
                            op(FunctionToTest::CalculateNormalizerBase64),
                            DEFAULT_EXPECTED,
                            b,
                            Some(DEFAULT_EXPECTED),
                            EXPECTED_FALSE,
                            DEFAULT_EXPECTED,
                        )
                    })
                    .boxed(),
            ),
            "calculate_normalizer_base_64 wrong u64 flag",
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn calculate_normalizer_base_64_rejects_wrong_u128_flag(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                arb_non_zero_u64()
                    .prop_map(|b| {
                        (
                            op(FunctionToTest::CalculateNormalizerBase64),
                            DEFAULT_EXPECTED,
                            u128::from(b),
                            Some(DEFAULT_EXPECTED),
                            EXPECTED_TRUE,
                            DEFAULT_EXPECTED,
                        )
                    })
                    .boxed(),
            ),
            "calculate_normalizer_base_64 wrong u128 flag",
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn estimate_quotient_digit_base_64(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (
                    any::<u64>()
                        .prop_filter("normalized divisor high word", |value| *value >= (1 << 63)),
                    any::<u64>(),
                    any::<u128>(),
                )
                    .prop_flat_map(|(b_high, b_low, a_low)| {
                        (0..b_high).prop_map(move |a_high| {
                            let a = (U256::from(a_high) << 128) | U256::from(a_low);
                            let b = (u128::from(b_high) << 64) | u128::from(b_low);
                            let quotient = (a / U256::from(b)).as_u128();

                            (
                                op(FunctionToTest::EstimateQuotientDigitBase64),
                                u128::from(a_high),
                                a_low,
                                Some(quotient),
                                EXPECTED_FALSE,
                                b,
                            )
                        })
                    })
                    .boxed(),
            ),
            "estimate_quotient_digit_base_64",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn estimate_quotient_digit_base_64_rejects_overflow(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (
                    any::<u64>().prop_filter("non-max normalized divisor high word", |value| {
                        *value >= (1 << 63) && *value < u64::MAX
                    }),
                    any::<u64>(),
                    any::<u128>(),
                )
                    .prop_flat_map(|(b_high, b_low, a_low)| {
                        ((b_high + 1)..=u64::MAX).prop_map(move |a_high| {
                            let a = (U256::from(a_high) << 128) | U256::from(a_low);
                            let b = (u128::from(b_high) << 64) | u128::from(b_low);
                            let quotient = (a / U256::from(b)).as_u128();

                            (
                                op(FunctionToTest::EstimateQuotientDigitBase64),
                                u128::from(a_high),
                                a_low,
                                Some(quotient),
                                EXPECTED_FALSE,
                                b,
                            )
                        })
                    })
                    .boxed(),
            ),
            "estimate_quotient_digit_base_64 overflow",
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn div_mod_128_64(fuzz_engine_builder: U128BasicMathFuzzEngineBuilder) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), arb_non_zero_u64())
                    .prop_map(|(a, b)| {
                        let b = u128::from(b);
                        (
                            op(FunctionToTest::DivMod128_64),
                            a,
                            b,
                            Some(a / b),
                            EXPECTED_FALSE,
                            a % b,
                        )
                    })
                    .boxed(),
            ),
            "div_mod_128_64",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn div_mod_128_64_rejects_zero_divisor(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                any::<u128>()
                    .prop_map(|a| {
                        (
                            op(FunctionToTest::DivMod128_64),
                            a,
                            DEFAULT_EXPECTED,
                            Some(DEFAULT_EXPECTED),
                            EXPECTED_FALSE,
                            DEFAULT_EXPECTED,
                        )
                    })
                    .boxed(),
            ),
            "div_mod_128_64 zero divisor",
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn div_mod_128(fuzz_engine_builder: U128BasicMathFuzzEngineBuilder) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), arb_non_zero_u128())
                    .prop_map(|(a, b)| {
                        (
                            op(FunctionToTest::DivMod128),
                            a,
                            b,
                            Some(a / b),
                            EXPECTED_FALSE,
                            a % b,
                        )
                    })
                    .boxed(),
            ),
            "div_mod_128",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn div_128(fuzz_engine_builder: U128BasicMathFuzzEngineBuilder) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (any::<u128>(), arb_non_zero_u128())
                    .prop_map(|(a, b)| {
                        (
                            op(FunctionToTest::Div128),
                            a,
                            b,
                            Some(a / b),
                            EXPECTED_FALSE,
                            DEFAULT_EXPECTED,
                        )
                    })
                    .boxed(),
            ),
            "div_128",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn div_128_rejects_zero_divisor(
        fuzz_engine_builder: U128BasicMathFuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_u128_basic_math_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                any::<u128>()
                    .prop_map(|a| {
                        (
                            op(FunctionToTest::Div128),
                            a,
                            DEFAULT_EXPECTED,
                            Some(DEFAULT_EXPECTED),
                            EXPECTED_FALSE,
                            DEFAULT_EXPECTED,
                        )
                    })
                    .boxed(),
            ),
            "div_128 zero divisor",
            Expect::AssertFailed,
        )
    }
}
