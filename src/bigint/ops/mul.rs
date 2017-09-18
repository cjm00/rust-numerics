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

        let carry = smul(&mut self.digits, rhs);

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


pub(crate) fn naive_mul(lhs: &BigInt, rhs: &BigInt) -> BigInt {
    let sign = lhs.sign * rhs.sign;
    if sign == Zero {return BigInt::zero()}

    let mut digits = vec![0; lhs.digits.len() + rhs.digits.len()];
    let mut carry: BigDigit = 0;

    for (i, l) in lhs.digits.iter().cloned().enumerate() {
        if l == 0 {continue}
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

/// Multiplies a slice by a constant, returning the carry.
pub(crate) fn smul(lhs: &mut [BigDigit], rhs: BigDigit) -> BigDigit {
        let rhs = rhs as DoubleBigDigit;
        let mut carry: BigDigit = 0;
        for d in lhs.iter_mut() {
            let [lo, hi] = lo_hi_digits((*d as DoubleBigDigit * rhs) + carry as DoubleBigDigit);
            *d = lo;
            carry = hi;
        }
        carry
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
