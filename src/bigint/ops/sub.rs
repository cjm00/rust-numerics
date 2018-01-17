use bigint::BigInt;
use bigint::Sign::*;
use bigint::digit::{BigDigit};
use bigint::digit::constants::DIGIT_MAX;
use bigint::ops::add;
use bigint::sign::Sign;

use std::ops::Sub;

impl Sub<BigInt> for BigInt {
    type Output = BigInt;

    fn sub(mut self, rhs: BigInt) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Positive, Positive) => {
                self.grow_to_hold(rhs.digits.len());
                if let Negative = ssub(&mut self.digits, &rhs.digits) {
                    -self.trimmed()
                } else {
                    self.trimmed()
                }
            }
            (Positive, Negative) => (self + -rhs),
            (Zero, _) => -rhs,
            (_, Zero) => self,
            (Negative, Positive) => -(-self + rhs),
            (Negative, Negative) => -(-self - -rhs),
        }
    }
}

impl<'a, 'b> Sub<&'a BigInt> for &'b BigInt {
    type Output = BigInt;

    fn sub(self, rhs: &'a BigInt) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Positive, Positive) => {
                let mut lhs = self.clone();
                lhs.grow_to_hold(rhs.digits.len());
                if let Negative = ssub(&mut lhs.digits, &rhs.digits) {
                    -lhs.trimmed()
                } else {
                    lhs.trimmed()
                }
            }
            (Positive, Negative) => (self.clone() - rhs),
            (Zero, _) => -rhs.clone(),
            (_, Zero) => self.clone(),
            (Negative, Positive) => -(-self.clone() + rhs),
            (Negative, Negative) => -(-self.clone() - -rhs.clone()),
        }
    }
}

impl<'a> Sub<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn sub(mut self, rhs: &'a BigInt) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Positive, Positive) => {
                self.grow_to_hold(rhs.digits.len());
                if let Negative = ssub(&mut self.digits, &rhs.digits) {
                    -self.trimmed()
                } else {
                    self.trimmed()
                }
            }
            (Positive, Negative) => {
                add::add_and_grow(&mut self, rhs);
                self
            }
            (Zero, _) => -rhs.clone(),
            (_, Zero) => self,
            (Negative, Positive) => -(-self + rhs),
            (Negative, Negative) => {
                self.grow_to_hold(rhs.digits.len());
                if let Negative = ssub(&mut self.digits, &rhs.digits) {
                    -self.trimmed()
                } else {
                    self.trimmed()
                }
            }
        }
    }
}


// TODO: Optimize!
/// "Slice subract", subtracts rhs from lhs in-place and returns
/// the sign of the result.
pub(crate) fn ssub(lhs: &mut [BigDigit], rhs: &[BigDigit]) -> Sign {
    assert!(lhs.len() >= rhs.len());
    let mut carry = false;

    {
        let (l_lo, l_hi) = lhs.split_at_mut(rhs.len());
        for (l, r) in l_lo.iter_mut().zip(rhs.iter().cloned()) {
            let (res, c) = l.overflowing_sub(r);
            let (res, d) = if carry { res.overflowing_sub(1) } else { (res, false) };
            *l = res;
            carry = c || d;
        }
        if carry {
            carry = dsub(l_hi, 1);
        }
    }

    if carry {
        for l in lhs.iter_mut() {
            *l = DIGIT_MAX - *l;
        }
        lhs[0] += 1;
    }

    if carry {
        Sign::Negative
    } else if all_zero(lhs) {
        Sign::Zero
    } else {
        Sign::Positive
    }

}

pub(crate) fn ssub_sign(lhs: &[BigDigit], rhs: &[BigDigit]) -> (Sign, Vec<BigDigit>) {
    debug_assert!(lhs.len() >= rhs.len());
    let mut output = lhs.to_owned();
    let sign = ssub(&mut output, rhs);
    (sign, output)
}

/// "Digit subtract", subtracts rhs from lhs in-place and returns whether or not a borrow and carry occurred.
pub(crate) fn dsub(lhs: &mut [BigDigit], rhs: BigDigit) -> bool {
    let mut carry = rhs;

    for ele in lhs.iter_mut() {
        if carry == 0 {
            return false;
        }
        let (res, c) = ele.overflowing_sub(carry);
        *ele = res;
        carry = c as BigDigit;
    }
    carry != 0
}

fn all_zero(s: &[BigDigit]) -> bool {
    s.iter().all(|&x| x == 0)
}


#[test]
fn dsub_test() {
    let mut foo = [0, 0, 5, 5];
    let res = [BigDigit::max_value(), BigDigit::max_value(), 4, 5];
    assert!(!dsub(&mut foo, 1));
    assert_eq!(foo, res);
}
