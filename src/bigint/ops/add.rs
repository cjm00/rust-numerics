use bigint::{BigInt, BigDigit, DoubleBigDigit};
use bigint::Sign::*;
use bigint::digit::{to_lo_hi};

use std::ops::Add;



impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(mut self, rhs: BigInt) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Positive, Positive) => {
                self.grow_to_hold(rhs.digits.len());
                let carry = sadd(&mut self.digits, &rhs.digits);
                self.digits.push(carry as BigDigit);
                self.trimmed()
            }
            (Positive, Negative) => self - (-rhs),
            (Zero, _) => rhs,
            (_, Zero) => self,
            (Negative, Positive) => rhs - self,
            (Negative, Negative) => -(-self + -rhs),
        }
    }
}


impl<'a, 'b> Add<&'b BigInt> for &'a BigInt {
    type Output = BigInt;
    fn add(self, rhs: &'b BigInt) -> Self::Output {
        if self >= rhs {
            let mut output = self.clone();
            add_and_grow(&mut output, rhs);
            output
        } else {
            let mut output = rhs.clone();
            add_and_grow(&mut output, self);
            output
        }
    }
}

impl<'a> Add<&'a BigInt> for BigInt {
    type Output = BigInt;
    fn add(mut self, rhs: &'a BigInt) -> Self::Output {
        add_and_grow(&mut self, rhs);
        self
    }
}

impl Add<BigDigit> for BigInt {
    type Output = BigInt;
    fn add(mut self, rhs: BigDigit) -> Self::Output {
        if self.is_zero() {
            self.digits.push(rhs);
            return self;
        }
        let carry = sadd_digit(&mut self.digits, rhs);
        if carry > 0 {
            self.digits.push(carry);
        }
        self
    }
}

pub(crate) fn add_and_grow(lhs: &mut BigInt, rhs: &BigInt) {
    lhs.grow_to_hold(rhs.digits.len());
    let grow = sadd(&mut lhs.digits, &rhs.digits);
    if grow > 0 {
        lhs.digits.push(grow);
    }
    lhs.trim()
}

// TODO: Optimize
pub(crate) fn sadd(lhs: &mut [BigDigit], rhs: &[BigDigit]) -> BigDigit {
    debug_assert!(lhs.len() >= rhs.len());
    let mut carry: BigDigit = 0;
    let (l_lo, l_hi) = lhs.split_at_mut(rhs.len());
    for (l, r) in l_lo.iter_mut().zip(rhs.iter().cloned()) {
        let [lo, hi] = to_lo_hi(*l as DoubleBigDigit + r as DoubleBigDigit + carry as DoubleBigDigit);
        *l = lo;
        carry = hi;
    }

    if carry != 0 {
        carry = sadd_digit(l_hi, carry);
    }
    carry
}

pub(crate) fn sadd_digit(lhs: &mut [BigDigit], rhs: BigDigit) -> BigDigit {
    let mut carry = rhs;

    for ele in lhs.iter_mut() {
        if carry == 0 {
            break;
        }
        let (res, c) = ele.overflowing_add(carry);
        *ele = res;
        carry = c as BigDigit;
    }
    carry
}


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
