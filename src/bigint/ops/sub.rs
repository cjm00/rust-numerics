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
pub(crate) fn ssub(lhs: &mut [BigDigit], rhs: &[BigDigit]) -> bool {
    assert!(lhs.len() >= rhs.len());
    let mut carry = false;

    for (l, r) in lhs.iter_mut().zip(rhs.iter().cloned()) {
        let (res, c) = l.overflowing_sub(r);
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

pub(crate) fn ssub_digit(lhs: &mut [BigDigit], rhs: BigDigit) -> bool {
    assert!(!lhs.is_empty());

    let (res, mut carry) = lhs[0].overflowing_sub(rhs);
    lhs[0] = res;

    let mut index = 1;
    while index < lhs.len() {
        if !carry {break}
        let (res, c) = lhs[index].overflowing_sub(1);
        lhs[index] = res;
        carry = c;
        index += 1;
    }
    carry
}


#[test]
fn ssub_digit_test() {
    let mut foo = [0, 0, 5, 5];
    let res = [BigDigit::max_value(), BigDigit::max_value(), 4, 5];
    assert!(!ssub_digit(&mut foo, 1));
    assert_eq!(foo, res);
}

