use bigint::{BigInt, BigDigit};
use bigint::Sign::*;

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
    fn add(self, rhs: &'b BigInt) -> BigInt {
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

impl Add<BigDigit> for BigInt {
    type Output = BigInt;
    fn add(mut self, rhs: BigDigit) -> Self::Output {
        if self.is_zero() {
            self.digits.push(rhs);
            return self;
        }
        let carry = sadd_digit(&mut self.digits, rhs);
        if carry {self.digits.push(1)}
        self
    }
}

fn add_and_grow(lhs: &mut BigInt, rhs: &BigInt) {
    lhs.grow_to_hold(rhs.digits.len());
    let grow = sadd(&mut lhs.digits, &rhs.digits);
    if grow {lhs.digits.push(1)}
    lhs.trim()
}

pub(crate) fn sadd(lhs: &mut [BigDigit], rhs: &[BigDigit]) -> bool {
    let mut carry = false;
    for (l, r) in lhs.iter_mut().zip(rhs.iter().cloned()) {
        let (res, c) = l.overflowing_add(r);
        let (res, d) = if carry {res.overflowing_add(1)} else {(res, false)};
        *l = res;
        carry = c || d;
    }
    carry
}

pub(crate) fn sadd_digit(lhs: &mut [BigDigit], rhs: BigDigit) -> bool {
    assert!(!lhs.is_empty());
    let mut carry = rhs;

    for ele in lhs.iter_mut() {
        if carry == 0 {return false;}
        let (res, c) = ele.overflowing_add(carry);
        *ele = res;
        carry = c as BigDigit;
    }
    carry != 0
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
