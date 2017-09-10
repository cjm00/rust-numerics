use bigint::{BigDigit, BigInt, DoubleBigDigit};
use bigint::Sign::*;

use bigint::digit::lo_hi_digits;

use std::ops::Mul;

impl Mul<BigInt> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> Self::Output {
        naive_mul(&self, &rhs)
    }
}

impl Mul<u32> for BigInt {
    type Output = BigInt;
    fn mul(mut self, rhs: u32) -> Self::Output {
        if self.is_zero() {
            return self;
        }

        let rhs = rhs as DoubleBigDigit;

        let mut carry: BigDigit = 0;
        for d in self.digits.iter_mut() {
            let [lo, hi] = lo_hi_digits((*d as DoubleBigDigit * rhs) + carry as DoubleBigDigit);
            *d = lo;
            carry = hi;
        }

        if carry != 0 {
            self.digits.push(carry);
        }

        self
    }
}

impl Mul<u64> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: u64) -> Self::Output {
        self * BigInt::from(rhs)
    }
}


pub fn naive_mul(lhs: &BigInt, rhs: &BigInt) -> BigInt {
    let sign = lhs.sign * rhs.sign;
    if sign == Zero {
        return BigInt::zero();
    };

    let mut digits = vec![0; lhs.digits.len() + rhs.digits.len()];
    let mut carry: BigDigit = 0;

    for (i, l) in lhs.digits.iter().cloned().enumerate() {
        if l == 0 {
            continue;
        }
        for (j, r) in rhs.digits.iter().cloned().enumerate() {
            let [lo, hi] = lo_hi_digits(
                l as DoubleBigDigit * r as DoubleBigDigit + digits[i + j] as DoubleBigDigit +
                    carry as DoubleBigDigit,
            );
            digits[i + j] = lo;
            if j + 1 != rhs.digits.len() {
                carry = hi;
            } else {
                carry = 0;
                digits[i + j + 1] = hi;
            }
        }
    }

    let mut out = BigInt { sign, digits };
    out.trim();
    out
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
    fn public_mul_bench_300_digits(z: &mut test::Bencher) {
        use std::str::FromStr;
        use bigint::BigInt;
        let a = BigInt::from_str(
            "5456284523795942428469132284583767671501937037838739553621048584\
             9697092671636555790748721976605446109138929406644601614207935832\
             9964386445517985764609355606748230433631464221297015136886306912\
             0150306437158735020650372401793455227707945470220316361023964964\
             94196228645882096543957791225647308272136068",
        ).unwrap();
        let b = BigInt::from_str(
            "8150042713655314982798403660314442131289517775633683156292882318\
             5421703900831693849991210570000636926440830281144478400477822470\
             0589121808067513932764450616107024538371323216631314068725835196\
             6885199417956362386912275659558457477517193882086534714263124255\
             28364107399545593257071086181824021275555532",
        ).unwrap();
        z.iter(|| &a + &b);
    }
}
