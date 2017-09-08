use super::BigInt;
use super::Sign::*;

use std::ops::Sub;

impl Sub<BigInt> for BigInt {
    type Output = BigInt;

    fn sub(mut self, rhs: BigInt) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Positive, Positive) => {
                sub::naive_sub(&mut self, &rhs);
                self
            }
            (Positive, Zero) => self,
            (Positive, Negative) => (self + -rhs),
            (Zero, _) => -rhs,
            (Negative, Positive) => -(-self + rhs),
            (Negative, Zero) => self,
            (Negative, Negative) => -(-self - -rhs),
        }
    }
}

pub(crate) mod sub {
    use super::BigInt;
    pub fn naive_sub(lhs: &mut BigInt, rhs: &BigInt) {
        if *lhs >= *rhs {
            let mut carry = false;

            for (l, r) in lhs.digits.iter_mut().zip(rhs.digits.iter().cloned()) {
                let (res, c) = l.overflowing_sub(r);
                let (res, d) = if carry {
                    res.overflowing_sub(1)
                } else {
                    (res, false)
                };
                *l = res;
                carry = c || d;
            }
        } else {
            let new_len = ::std::cmp::max(lhs.digits.len(), rhs.digits.len());
            lhs.digits.resize(new_len, 0);

            let mut carry = false;

            for (l, r) in lhs.digits.iter_mut().zip(rhs.digits.iter().cloned()) {
                let (res, c) = r.overflowing_sub(*l);
                let (res, d) = if carry {
                    res.overflowing_sub(1)
                } else {
                    (res, false)
                };
                *l = res;
                carry = c || d;
            }
            lhs.negate();
        }

        lhs.trim();
    }


}