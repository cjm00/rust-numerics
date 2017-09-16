use bigint::BigInt;
use bigint::Sign::*;
use bigint::digit::BigDigit;
use bigint::digit::constants::DIGIT_MAX;

use std::ops::Sub;


impl Sub<BigInt> for BigInt {
    type Output = BigInt;

    fn sub(mut self, rhs: BigInt) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Positive, Positive) => {
                self.grow_to_hold(rhs.digits.len());
                if ssub(&mut self.digits, &rhs.digits) {-self.trimmed()} else {self.trimmed()}}
            (Positive, Negative) => (self + -rhs),
            (Zero, _) => -rhs,
            (_, Zero) => self,
            (Negative, Positive) => -(-self + rhs),
            (Negative, Negative) => -(-self - -rhs),
        }
    }
}


/// "Slice subract", subtracts lhs from rhs in-place and returns whether or not a borrow and complement occurred
#[inline]
pub(crate) fn ssub(lhs: &mut [BigDigit], rhs: &[BigDigit]) -> bool {
    let mut carry = false;

    for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
        let (res, c) = l.overflowing_sub(*r);
        let (res, d) = if carry {res.overflowing_sub(1)} else {(res, false)};
        *l = res;
        carry = c || d;
    }

    if carry {
        for l in lhs.iter_mut() {
            *l = DIGIT_MAX - *l;
        }
        lhs[0] += 1;
    }

    carry

}


