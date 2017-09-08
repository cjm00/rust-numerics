use bigint::{BigInt, Sign};
use bigint::digit::{BigDigit, DoubleBigDigit};

pub mod add;
pub mod sub;
pub mod mul;
pub mod div;



#[test]
fn add_test_1() {
    use bigint::sign::Sign;

    let a: BigDigit = 0;
    let a = a.wrapping_sub(1);

    let a_big = BigInt::from(a);
    let b_big = BigInt::from(a);
    let c_big = a_big + b_big;

    let z: BigDigit = 0;
    let z = z.wrapping_sub(2);

    let c_fixed = BigInt {
        sign: Sign::Positive,
        digits: vec![z, 1],
    };

    assert_eq!(c_big, c_fixed);
}

#[cfg(all(target_pointer_width = "64", not(feature = "thicc_ints")))]
#[test]
fn scalar_mul_test_1() {
    use bigint::sign::Sign;

    let y: u32 = 915327;

    let a = BigInt {
        sign: Sign::Positive,
        digits: vec![3059078384, 2360247638, 2634550291, 6],
    };
    let b = BigInt {
        sign: Sign::Positive,
        digits: vec![356004624, 4070707789, 1201864523, 6053427],
    };

    assert_eq!(a * y, b);
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    extern crate test;
    
    #[bench]
    fn public_add_bench_100(z: &mut test::Bencher) {
        use std::str::FromStr;
        use ::bigint::BigInt;
        let a = BigInt::from_str("5605552357266437729280504134296683206097961396781567121352972852719206640545169925410820532634965117").unwrap();
        let b = BigInt::from_str("4247965229701346452175430137483132154566023748284704589458094199981810537483023545986277408132789499").unwrap();
        z.iter(|| &a + &b);
    }
}