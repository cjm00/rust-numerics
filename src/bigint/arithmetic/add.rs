use super::{BigInt, BigDigit};
use super::Sign::*;

use std::ops::Add;


impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(mut self, rhs: BigInt) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Positive, Positive) => {
                naive_add(&mut self, &rhs);
                self
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
            naive_add(&mut output, rhs);
            output
        } else {
            let mut output = rhs.clone();
            naive_add(&mut output, self);
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
        let (res, mut carry) = self.digits[0].overflowing_add(rhs);
        self.digits[0] = res;

        let mut index = 1usize;
        while carry && (index < self.digits.len()) {
            let (r, c) = self.digits[index].overflowing_add(1);
            self.digits[index] = r;
            carry = c;
            index += 1;
        }

        if carry {
            self.digits.push(1);
        }

        self
    }
}


    pub fn naive_add(lhs: &mut BigInt, rhs: &BigInt) {
        let new_len = ::std::cmp::max(lhs.digits.len(), rhs.digits.len());
        lhs.digits.resize(new_len, 0);
        let mut carry = false;
        for (l, r) in lhs.digits.iter_mut().zip(rhs.digits.iter().cloned()) {
            let (res, c) = l.overflowing_add(r);
            let (res, d) = if carry {
                res.overflowing_add(1)
            } else {
                (res, false)
            };
            *l = res;
            carry = c || d;
        }
        if carry {
            lhs.digits.push(1)
        }
        lhs.trim();
    }

    pub fn ripple_add(lhs: &mut [BigDigit], rhs: BigDigit) {
        assert!(lhs.len() >= 2);
        let (res, mut carry) = lhs[0].overflowing_add(rhs);
        lhs[0] = res;
        let mut index = 1;
        while carry {
            let (r, c) = lhs[index].overflowing_add(1);
            carry = c;
            lhs[index] = r;
            index += 1;
        }
    }

