mod common;

use num_bigint::BigUint;
use num_traits::Zero;
use rand::{RngCore, rngs::OsRng};
use secp256k1_zkp::{PublicKey, Secp256k1, SecretKey, rand::rngs::OsRng as SecpOsRng};

use common::core::{Expect, run};

use simplicityhl_std::artifacts::secp256k1_operations_test::Secp256k1OperationsTestProgram;
use simplicityhl_std::artifacts::secp256k1_operations_test::derived_secp256k1_operations_test::{
    Secp256k1OperationsTestArguments, Secp256k1OperationsTestWitness,
};

enum FunctionToTest {
    GeToPoint,
    PointToGej,
    FeSub,
    ScalarSub,
    GejSub,
    FeEq,
    ScalarEq,
    GeEq,
    GejPointEq,
    SafeGejNormalize,
}

#[inline]
fn op(o: FunctionToTest) -> u8 {
    o as u8
}

const DEFAULT_UINT: [u8; 32] = [0u8; 32];
const DEFAULT_GE: ([u8; 32], [u8; 32]) = ([0u8; 32], [0u8; 32]);
const DEFAULT_GEJ: (([u8; 32], [u8; 32]), [u8; 32]) = (([0u8; 32], [0u8; 32]), [0u8; 32]);
const DEFAULT_POINT: (u8, [u8; 32]) = (0, [0u8; 32]);

// FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
const SECP_P: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F,
];

// FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
const SECP_N: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
    0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41,
];

fn program() -> Secp256k1OperationsTestProgram {
    Secp256k1OperationsTestProgram::new(Secp256k1OperationsTestArguments {})
}

#[allow(clippy::too_many_arguments)]
fn build_witness(
    function: u8,
    first_uint: [u8; 32],
    second_uint: [u8; 32],
    first_ge: ([u8; 32], [u8; 32]),
    second_ge: ([u8; 32], [u8; 32]),
    first_gej: (([u8; 32], [u8; 32]), [u8; 32]),
    second_gej: (([u8; 32], [u8; 32]), [u8; 32]),
    first_point: (u8, [u8; 32]),
    expected_uint: [u8; 32],
    expected_ge: ([u8; 32], [u8; 32]),
    expected_gej: (([u8; 32], [u8; 32]), [u8; 32]),
    expected_point: (u8, [u8; 32]),
) -> Secp256k1OperationsTestWitness {
    Secp256k1OperationsTestWitness {
        function_index: function,
        first_uint,
        second_uint,
        first_ge,
        second_ge,
        first_gej,
        second_gej,
        first_point,
        expected_uint,
        expected_ge,
        expected_gej,
        expected_point,
    }
}

fn secp_p() -> BigUint {
    BigUint::from_bytes_be(&SECP_P)
}
fn secp_n() -> BigUint {
    BigUint::from_bytes_be(&SECP_N)
}

fn to_32_be(x: &BigUint) -> [u8; 32] {
    let b = x.to_bytes_be();

    let mut out = [0u8; 32];
    out[32 - b.len()..].copy_from_slice(&b);

    out
}

fn mod_p(x: &BigUint) -> BigUint {
    x % secp_p()
}
fn mod_n(x: &BigUint) -> BigUint {
    x % secp_n()
}

fn fe_sub_ref(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let p = secp_p();
    let a = BigUint::from_bytes_be(&a) % &p;
    let b = BigUint::from_bytes_be(&b) % &p;

    to_32_be(&((a + &p - b) % &p))
}

fn scalar_sub_ref(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let n = secp_n();
    let a = BigUint::from_bytes_be(&a) % &n;
    let b = BigUint::from_bytes_be(&b) % &n;

    to_32_be(&((a + &n - b) % &n))
}

fn fe_negate_ref(a: [u8; 32]) -> [u8; 32] {
    let p = secp_p();
    let a = BigUint::from_bytes_be(&a) % &p;

    if a.is_zero() {
        [0u8; 32]
    } else {
        to_32_be(&(&p - &a))
    }
}

fn fe_mul_ref(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let p = secp_p();
    let a = BigUint::from_bytes_be(&a);
    let b = BigUint::from_bytes_be(&b);

    to_32_be(&((a * b) % &p))
}

// Randomness helpers
fn random_uint_bytes() -> [u8; 32] {
    let mut b = [0u8; 32];
    OsRng.fill_bytes(&mut b);

    b
}

fn random_fe_bytes() -> [u8; 32] {
    to_32_be(&mod_p(&BigUint::from_bytes_be(&random_uint_bytes())))
}

fn random_scalar_bytes() -> [u8; 32] {
    to_32_be(&mod_n(&BigUint::from_bytes_be(&random_uint_bytes())))
}

fn random_ge_bytes() -> ([u8; 32], [u8; 32]) {
    let secp = Secp256k1::new();
    let sk = SecretKey::new(&mut SecpOsRng);
    let pk = PublicKey::from_secret_key(&secp, &sk);
    let ser = pk.serialize_uncompressed(); // 0x04 || x(32) || y(32)

    let mut x = [0u8; 32];
    x.copy_from_slice(&ser[1..33]);

    let mut y = [0u8; 32];
    y.copy_from_slice(&ser[33..65]);

    (x, y)
}

fn ge_to_gej(ge: ([u8; 32], [u8; 32])) -> (([u8; 32], [u8; 32]), [u8; 32]) {
    let mut one = [0u8; 32];
    one[31] = 1;

    (ge, one)
}

fn compress(ge: ([u8; 32], [u8; 32])) -> (u8, [u8; 32]) {
    (ge.1[31] & 1, ge.0)
}

fn pk_to_gej(pk: &PublicKey) -> (([u8; 32], [u8; 32]), [u8; 32]) {
    let ser = pk.serialize_uncompressed();

    let mut x = [0u8; 32];
    x.copy_from_slice(&ser[1..33]);

    let mut y = [0u8; 32];
    y.copy_from_slice(&ser[33..65]);

    ge_to_gej((x, y))
}

mod secp256k1_operations_tests {
    use super::*;

    // 0. ge_to_point
    #[simplex::test]
    fn ge_to_point_matches_parity(context: simplex::TestContext) -> anyhow::Result<()> {
        // Sample one on-curve point; whatever parity it has, that's what we expect
        // ge_to_point to produce. Covers both parities across repeated runs.
        let ge = random_ge_bytes();
        let expected_parity = ge.1[31] & 1; // 0 (even y) or 1 (odd y)

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::GeToPoint),
                DEFAULT_UINT,
                DEFAULT_UINT,
                ge,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_GEJ,
                DEFAULT_POINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GEJ,
                (expected_parity, ge.0),
            ),
            Expect::Ok,
        )
    }

    // 1. point_to_gej
    #[simplex::test]
    fn point_to_gej_roundtrip(context: simplex::TestContext) -> anyhow::Result<()> {
        let ge = random_ge_bytes();
        let point = compress(ge);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::PointToGej),
                DEFAULT_UINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_GEJ,
                point,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GEJ,
                point,
            ),
            Expect::Ok,
        )
    }

    // 2. fe_sub
    #[simplex::test]
    fn fe_sub_self_is_zero(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = random_fe_bytes();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FeSub),
                a,
                a,
                DEFAULT_GE,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_GEJ,
                DEFAULT_POINT,
                [0u8; 32],
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_POINT,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn fe_sub_matches_reference(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = random_fe_bytes();
        let b = random_fe_bytes();
        let exp = fe_sub_ref(a, b);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FeSub),
                a,
                b,
                DEFAULT_GE,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_GEJ,
                DEFAULT_POINT,
                exp,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_POINT,
            ),
            Expect::Ok,
        )
    }

    // 3. scalar_sub
    #[simplex::test]
    fn scalar_sub_matches_reference(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = random_scalar_bytes();
        let b = random_scalar_bytes();
        let exp = scalar_sub_ref(a, b);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::ScalarSub),
                a,
                b,
                DEFAULT_GE,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_GEJ,
                DEFAULT_POINT,
                exp,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_POINT,
            ),
            Expect::Ok,
        )
    }

    // 4. gej_sub
    #[simplex::test]
    fn gej_sub_matches_reference(context: simplex::TestContext) -> anyhow::Result<()> {
        let secp = Secp256k1::new();
        let sk_p = SecretKey::new(&mut SecpOsRng);
        let sk_q = SecretKey::new(&mut SecpOsRng);

        let p = PublicKey::from_secret_key(&secp, &sk_p);
        let q = PublicKey::from_secret_key(&secp, &sk_q);
        let diff = p.combine(&q.negate(&secp)).expect("p - q non-infinity");

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::GejSub),
                DEFAULT_UINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GE,
                pk_to_gej(&p),
                pk_to_gej(&q),
                DEFAULT_POINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                pk_to_gej(&diff),
                DEFAULT_POINT,
            ),
            Expect::Ok,
        )
    }

    // 5. fe_eq
    #[simplex::test]
    fn fe_eq_reflexive(context: simplex::TestContext) -> anyhow::Result<()> {
        let a = random_fe_bytes();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FeEq),
                a,
                a,
                DEFAULT_GE,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_GEJ,
                DEFAULT_POINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_POINT,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn fe_eq_zero_and_p_are_equal(context: simplex::TestContext) -> anyhow::Result<()> {
        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::FeEq),
                [0u8; 32],
                SECP_P,
                DEFAULT_GE,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_GEJ,
                DEFAULT_POINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_POINT,
            ),
            Expect::Ok,
        )
    }

    // 6. scalar_eq
    #[simplex::test]
    fn scalar_eq_s_and_s_plus_n_are_equal(context: simplex::TestContext) -> anyhow::Result<()> {
        // s = 5. s + n has no carry past the low byte.
        let mut s = [0u8; 32];
        s[31] = 5;

        let mut s_plus_n = SECP_N;
        s_plus_n[31] = s_plus_n[31].wrapping_add(5);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::ScalarEq),
                s,
                s_plus_n,
                DEFAULT_GE,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_GEJ,
                DEFAULT_POINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_POINT,
            ),
            Expect::Ok,
        )
    }

    // 7. ge_eq
    #[simplex::test]
    fn ge_eq_rejects_negation(context: simplex::TestContext) -> anyhow::Result<()> {
        let ge = random_ge_bytes();
        let neg_y = fe_negate_ref(ge.1);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::GeEq),
                DEFAULT_UINT,
                DEFAULT_UINT,
                ge,
                (ge.0, neg_y),
                DEFAULT_GEJ,
                DEFAULT_GEJ,
                DEFAULT_POINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_POINT,
            ),
            Expect::AssertFailed,
        )
    }

    // 8. gej_point_eq
    #[simplex::test]
    fn gej_point_eq_rescaled(context: simplex::TestContext) -> anyhow::Result<()> {
        // (λ²X, λ³Y, λZ) is the same affine point as (X, Y, 1).
        let ge = random_ge_bytes();
        let lambda = random_fe_bytes();
        let l2 = fe_mul_ref(lambda, lambda);
        let l3 = fe_mul_ref(l2, lambda);
        let g = ((fe_mul_ref(ge.0, l2), fe_mul_ref(ge.1, l3)), lambda);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::GejPointEq),
                DEFAULT_UINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GE,
                g,
                DEFAULT_GEJ,
                compress(ge),
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_POINT,
            ),
            Expect::Ok,
        )
    }

    #[simplex::test]
    fn gej_point_eq_rejects_negation(context: simplex::TestContext) -> anyhow::Result<()> {
        // Property under test: gej_point_eq(P, -P) == false.
        //
        // Construction:
        //   ge          = random on-curve point P = (x, y)
        //   (parity, x) = compressed form of +P
        //   ge_to_gej(ge)      = Gej encoding of +P
        //   (parity ^ 1, x)    = compressed form of -P
        //
        // decompress((parity ^ 1, x)) inside gej_point_eq recovers (x, -y) = -P.
        // Since P ≠ -P on secp256k1, the equivalence must return false.
        // expected_bool = false locks that in.
        let ge = random_ge_bytes();
        let (parity, x) = compress(ge);

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::GejPointEq),
                DEFAULT_UINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GE,
                ge_to_gej(ge),
                DEFAULT_GEJ,
                (parity ^ 1, x),
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GEJ,
                DEFAULT_POINT,
            ),
            Expect::AssertFailed,
        )
    }

    // 9. safe_gej_normalize
    #[simplex::test]
    fn safe_gej_normalize_roundtrip(context: simplex::TestContext) -> anyhow::Result<()> {
        let ge = random_ge_bytes();

        run(
            &context,
            program(),
            build_witness(
                op(FunctionToTest::SafeGejNormalize),
                DEFAULT_UINT,
                DEFAULT_UINT,
                DEFAULT_GE,
                DEFAULT_GE,
                ge_to_gej(ge),
                DEFAULT_GEJ,
                DEFAULT_POINT,
                DEFAULT_UINT,
                ge,
                DEFAULT_GEJ,
                DEFAULT_POINT,
            ),
            Expect::Ok,
        )
    }
}

mod secp256k1_operations_tests_fuzz {
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

    type Ge = ([u8; 32], [u8; 32]);
    type Gej = (Ge, [u8; 32]);
    type Point = (u8, [u8; 32]);
    type Secp256k1FuzzEngineBuilder = FuzzEngineBuilder<
        Secp256k1OperationsTestProgram,
        Secp256k1OperationsTestArguments,
        Secp256k1OperationsTestWitness,
    >;

    #[derive(Clone, Copy, Debug)]
    struct Secp256k1FuzzCase {
        function: u8,
        first_uint: [u8; 32],
        second_uint: [u8; 32],
        first_ge: Ge,
        second_ge: Ge,
        first_gej: Gej,
        second_gej: Gej,
        first_point: Point,
        expected_uint: [u8; 32],
        expected_ge: Ge,
        expected_gej: Gej,
        expected_point: Point,
    }

    fn initial_transaction() -> FinalTransaction {
        let mut tx = FinalTransaction::new();
        tx.add_input(PartialInput::new(UTXO::default()), RequiredSignature::None);
        tx
    }

    fn transaction_builder() -> Result<FinalTransactionBuilder, FuzzError> {
        FinalTransactionBuilder::new(initial_transaction(), [PROGRAM_TARGET])
    }

    fn arb_fe() -> impl Strategy<Value = [u8; 32]> {
        any::<[u8; 32]>().prop_filter("valid secp256k1 field element", |value| *value < SECP_P)
    }

    fn arb_non_zero_fe() -> impl Strategy<Value = [u8; 32]> {
        arb_fe().prop_filter("non-zero secp256k1 field element", |value| {
            *value != [0; 32]
        })
    }

    fn arb_scalar() -> impl Strategy<Value = [u8; 32]> {
        any::<[u8; 32]>().prop_filter("valid secp256k1 scalar", |value| *value < SECP_N)
    }

    fn arb_secret_key() -> impl Strategy<Value = SecretKey> {
        any::<[u8; 32]>().prop_filter_map("valid secp256k1 secret key", |bytes| {
            SecretKey::from_slice(&bytes).ok()
        })
    }

    fn ge_from_secret_key(secret_key: SecretKey) -> Ge {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let serialized = public_key.serialize_uncompressed();

        let mut x = [0; 32];
        x.copy_from_slice(&serialized[1..33]);

        let mut y = [0; 32];
        y.copy_from_slice(&serialized[33..65]);

        (x, y)
    }

    fn arb_ge() -> impl Strategy<Value = Ge> {
        arb_secret_key().prop_map(ge_from_secret_key)
    }

    fn fuzz_strategy(
        cases: BoxedStrategy<Secp256k1FuzzCase>,
    ) -> BoxedStrategy<(Arguments, WitnessValues)> {
        FuzzStrategyBuilder::<
            Secp256k1OperationsTestArguments,
            Secp256k1OperationsTestWitness,
            _,
        >::new()
        .with_custom_strategy(cases.prop_map(|case| {
            let arguments: Arguments = Secp256k1OperationsTestArguments {}.into();
            let witness: WitnessValues = build_witness(
                case.function,
                case.first_uint,
                case.second_uint,
                case.first_ge,
                case.second_ge,
                case.first_gej,
                case.second_gej,
                case.first_point,
                case.expected_uint,
                case.expected_ge,
                case.expected_gej,
                case.expected_point,
            )
            .into();

            (arguments, witness)
        }))
        .build()
    }

    fn run_secp256k1_fuzz(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
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

    #[simplex::fuzz]
    fn ge_to_point_matches_parity(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                arb_ge()
                    .prop_map(|ge| Secp256k1FuzzCase {
                        function: op(FunctionToTest::GeToPoint),
                        first_uint: DEFAULT_UINT,
                        second_uint: DEFAULT_UINT,
                        first_ge: ge,
                        second_ge: DEFAULT_GE,
                        first_gej: DEFAULT_GEJ,
                        second_gej: DEFAULT_GEJ,
                        first_point: DEFAULT_POINT,
                        expected_uint: DEFAULT_UINT,
                        expected_ge: DEFAULT_GE,
                        expected_gej: DEFAULT_GEJ,
                        expected_point: compress(ge),
                    })
                    .boxed(),
            ),
            "ge_to_point",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn point_to_gej_roundtrip(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                arb_ge()
                    .prop_map(|ge| {
                        let point = compress(ge);
                        Secp256k1FuzzCase {
                            function: op(FunctionToTest::PointToGej),
                            first_uint: DEFAULT_UINT,
                            second_uint: DEFAULT_UINT,
                            first_ge: DEFAULT_GE,
                            second_ge: DEFAULT_GE,
                            first_gej: DEFAULT_GEJ,
                            second_gej: DEFAULT_GEJ,
                            first_point: point,
                            expected_uint: DEFAULT_UINT,
                            expected_ge: DEFAULT_GE,
                            expected_gej: DEFAULT_GEJ,
                            expected_point: point,
                        }
                    })
                    .boxed(),
            ),
            "point_to_gej",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn fe_sub_matches_reference(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (arb_fe(), arb_fe())
                    .prop_map(|(a, b)| Secp256k1FuzzCase {
                        function: op(FunctionToTest::FeSub),
                        first_uint: a,
                        second_uint: b,
                        first_ge: DEFAULT_GE,
                        second_ge: DEFAULT_GE,
                        first_gej: DEFAULT_GEJ,
                        second_gej: DEFAULT_GEJ,
                        first_point: DEFAULT_POINT,
                        expected_uint: fe_sub_ref(a, b),
                        expected_ge: DEFAULT_GE,
                        expected_gej: DEFAULT_GEJ,
                        expected_point: DEFAULT_POINT,
                    })
                    .boxed(),
            ),
            "fe_sub",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn scalar_sub_matches_reference(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (arb_scalar(), arb_scalar())
                    .prop_map(|(a, b)| Secp256k1FuzzCase {
                        function: op(FunctionToTest::ScalarSub),
                        first_uint: a,
                        second_uint: b,
                        first_ge: DEFAULT_GE,
                        second_ge: DEFAULT_GE,
                        first_gej: DEFAULT_GEJ,
                        second_gej: DEFAULT_GEJ,
                        first_point: DEFAULT_POINT,
                        expected_uint: scalar_sub_ref(a, b),
                        expected_ge: DEFAULT_GE,
                        expected_gej: DEFAULT_GEJ,
                        expected_point: DEFAULT_POINT,
                    })
                    .boxed(),
            ),
            "scalar_sub",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn gej_sub_matches_reference(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (arb_secret_key(), arb_secret_key())
                    .prop_filter("distinct secp256k1 points", |(p, q)| p != q)
                    .prop_map(|(p_secret, q_secret)| {
                        let secp = Secp256k1::new();
                        let p = PublicKey::from_secret_key(&secp, &p_secret);
                        let q = PublicKey::from_secret_key(&secp, &q_secret);
                        let difference = p
                            .combine(&q.negate(&secp))
                            .expect("distinct secp256k1 points have a non-infinite difference");

                        Secp256k1FuzzCase {
                            function: op(FunctionToTest::GejSub),
                            first_uint: DEFAULT_UINT,
                            second_uint: DEFAULT_UINT,
                            first_ge: DEFAULT_GE,
                            second_ge: DEFAULT_GE,
                            first_gej: pk_to_gej(&p),
                            second_gej: pk_to_gej(&q),
                            first_point: DEFAULT_POINT,
                            expected_uint: DEFAULT_UINT,
                            expected_ge: DEFAULT_GE,
                            expected_gej: pk_to_gej(&difference),
                            expected_point: DEFAULT_POINT,
                        }
                    })
                    .boxed(),
            ),
            "gej_sub",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn fe_eq_is_reflexive(fuzz_engine_builder: Secp256k1FuzzEngineBuilder) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                arb_fe()
                    .prop_map(|value| Secp256k1FuzzCase {
                        function: op(FunctionToTest::FeEq),
                        first_uint: value,
                        second_uint: value,
                        first_ge: DEFAULT_GE,
                        second_ge: DEFAULT_GE,
                        first_gej: DEFAULT_GEJ,
                        second_gej: DEFAULT_GEJ,
                        first_point: DEFAULT_POINT,
                        expected_uint: DEFAULT_UINT,
                        expected_ge: DEFAULT_GE,
                        expected_gej: DEFAULT_GEJ,
                        expected_point: DEFAULT_POINT,
                    })
                    .boxed(),
            ),
            "fe_eq",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn scalar_eq_is_reflexive(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                arb_scalar()
                    .prop_map(|value| Secp256k1FuzzCase {
                        function: op(FunctionToTest::ScalarEq),
                        first_uint: value,
                        second_uint: value,
                        first_ge: DEFAULT_GE,
                        second_ge: DEFAULT_GE,
                        first_gej: DEFAULT_GEJ,
                        second_gej: DEFAULT_GEJ,
                        first_point: DEFAULT_POINT,
                        expected_uint: DEFAULT_UINT,
                        expected_ge: DEFAULT_GE,
                        expected_gej: DEFAULT_GEJ,
                        expected_point: DEFAULT_POINT,
                    })
                    .boxed(),
            ),
            "scalar_eq",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn ge_eq_is_reflexive(fuzz_engine_builder: Secp256k1FuzzEngineBuilder) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                arb_ge()
                    .prop_map(|ge| Secp256k1FuzzCase {
                        function: op(FunctionToTest::GeEq),
                        first_uint: DEFAULT_UINT,
                        second_uint: DEFAULT_UINT,
                        first_ge: ge,
                        second_ge: ge,
                        first_gej: DEFAULT_GEJ,
                        second_gej: DEFAULT_GEJ,
                        first_point: DEFAULT_POINT,
                        expected_uint: DEFAULT_UINT,
                        expected_ge: DEFAULT_GE,
                        expected_gej: DEFAULT_GEJ,
                        expected_point: DEFAULT_POINT,
                    })
                    .boxed(),
            ),
            "ge_eq",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn ge_eq_rejects_negation(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                arb_ge()
                    .prop_map(|ge| Secp256k1FuzzCase {
                        function: op(FunctionToTest::GeEq),
                        first_uint: DEFAULT_UINT,
                        second_uint: DEFAULT_UINT,
                        first_ge: ge,
                        second_ge: (ge.0, fe_negate_ref(ge.1)),
                        first_gej: DEFAULT_GEJ,
                        second_gej: DEFAULT_GEJ,
                        first_point: DEFAULT_POINT,
                        expected_uint: DEFAULT_UINT,
                        expected_ge: DEFAULT_GE,
                        expected_gej: DEFAULT_GEJ,
                        expected_point: DEFAULT_POINT,
                    })
                    .boxed(),
            ),
            "ge_eq negation",
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn gej_point_eq_matches_scaled_point(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (arb_ge(), arb_non_zero_fe())
                    .prop_map(|(ge, lambda)| {
                        let lambda_squared = fe_mul_ref(lambda, lambda);
                        let lambda_cubed = fe_mul_ref(lambda_squared, lambda);

                        Secp256k1FuzzCase {
                            function: op(FunctionToTest::GejPointEq),
                            first_uint: DEFAULT_UINT,
                            second_uint: DEFAULT_UINT,
                            first_ge: DEFAULT_GE,
                            second_ge: DEFAULT_GE,
                            first_gej: (
                                (
                                    fe_mul_ref(ge.0, lambda_squared),
                                    fe_mul_ref(ge.1, lambda_cubed),
                                ),
                                lambda,
                            ),
                            second_gej: DEFAULT_GEJ,
                            first_point: compress(ge),
                            expected_uint: DEFAULT_UINT,
                            expected_ge: DEFAULT_GE,
                            expected_gej: DEFAULT_GEJ,
                            expected_point: DEFAULT_POINT,
                        }
                    })
                    .boxed(),
            ),
            "gej_point_eq",
            Expect::Ok,
        )
    }

    #[simplex::fuzz]
    fn gej_point_eq_rejects_negation(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                arb_ge()
                    .prop_map(|ge| {
                        let (parity, x) = compress(ge);
                        Secp256k1FuzzCase {
                            function: op(FunctionToTest::GejPointEq),
                            first_uint: DEFAULT_UINT,
                            second_uint: DEFAULT_UINT,
                            first_ge: DEFAULT_GE,
                            second_ge: DEFAULT_GE,
                            first_gej: ge_to_gej(ge),
                            second_gej: DEFAULT_GEJ,
                            first_point: (parity ^ 1, x),
                            expected_uint: DEFAULT_UINT,
                            expected_ge: DEFAULT_GE,
                            expected_gej: DEFAULT_GEJ,
                            expected_point: DEFAULT_POINT,
                        }
                    })
                    .boxed(),
            ),
            "gej_point_eq negation",
            Expect::AssertFailed,
        )
    }

    #[simplex::fuzz]
    fn safe_gej_normalize_matches_scaled_point(
        fuzz_engine_builder: Secp256k1FuzzEngineBuilder,
    ) -> anyhow::Result<()> {
        run_secp256k1_fuzz(
            fuzz_engine_builder,
            fuzz_strategy(
                (arb_ge(), arb_non_zero_fe())
                    .prop_map(|(ge, lambda)| {
                        let lambda_squared = fe_mul_ref(lambda, lambda);
                        let lambda_cubed = fe_mul_ref(lambda_squared, lambda);

                        Secp256k1FuzzCase {
                            function: op(FunctionToTest::SafeGejNormalize),
                            first_uint: DEFAULT_UINT,
                            second_uint: DEFAULT_UINT,
                            first_ge: DEFAULT_GE,
                            second_ge: DEFAULT_GE,
                            first_gej: (
                                (
                                    fe_mul_ref(ge.0, lambda_squared),
                                    fe_mul_ref(ge.1, lambda_cubed),
                                ),
                                lambda,
                            ),
                            second_gej: DEFAULT_GEJ,
                            first_point: DEFAULT_POINT,
                            expected_uint: DEFAULT_UINT,
                            expected_ge: ge,
                            expected_gej: DEFAULT_GEJ,
                            expected_point: DEFAULT_POINT,
                        }
                    })
                    .boxed(),
            ),
            "safe_gej_normalize",
            Expect::Ok,
        )
    }
}
