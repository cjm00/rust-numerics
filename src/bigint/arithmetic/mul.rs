use super::{BigInt, BigDigit, DoubleBigDigit};
use super::Sign::*;

use super::add::ripple_add;

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
            let [lo, hi] =
                BigInt::lo_hi_digits((*d as DoubleBigDigit * rhs) + carry as DoubleBigDigit);
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

        let mut digits = vec![0; lhs.digits.len() + rhs.digits.len() + 1];

        for (i, l) in lhs.digits.iter().cloned().enumerate() {
            for (j, r) in rhs.digits.iter().cloned().enumerate() {
                let [lo, hi] = BigInt::lo_hi_digits(l as DoubleBigDigit * r as DoubleBigDigit);
                ripple_add(&mut digits[i + j..], lo);
                ripple_add(&mut digits[i + j + 1..], hi);
            }
        }

        let mut out = BigInt { sign, digits };
        out.trim();
        out
    }


