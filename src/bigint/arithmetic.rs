use bigint::{BigInt, Sign};
use bigint::digit::{BigDigit, DoubleBigDigit};

use std::ops::{Add, Mul, Neg};

impl Neg for BigInt {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.sign = match self.sign {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
            Sign::Zero => Sign::Zero,
        };

        self
    }
}


impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Sign::Positive, Sign::Positive) => add::strict_pos_overflow_add(self, rhs),
            _ => unimplemented!(),
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

impl Mul<BigDigit> for BigInt {
    type Output = BigInt;
    fn mul(mut self, rhs: BigDigit) -> Self::Output {
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

pub(crate) mod add {
    use super::BigInt;
    use std::iter;

    pub fn strict_pos_overflow_add(mut lhs: BigInt, mut rhs: BigInt) -> BigInt {

        let l_len = lhs.digits.len();
        let r_len = rhs.digits.len();

        if lhs.digits.len() >= rhs.digits.len() {
            let mut carry = false;
            for (l, r) in lhs.digits.iter_mut().zip(
                rhs.digits
                    .iter()
                    .cloned()
                    .chain(iter::repeat(0).take(l_len - r_len)),
            ) {
                let (res, c) = l.overflowing_add(r);
                if carry {
                    let (c_res, d) = res.overflowing_add(1);
                    carry = c || d;
                    *l = c_res;
                } else {
                    *l = res;
                    carry = c;
                }
            }
            if carry {
                lhs.digits.push(1)
            }

            lhs
        } else {
            let mut carry = false;
            for (l, r) in rhs.digits.iter_mut().zip(
                lhs.digits
                    .iter()
                    .cloned()
                    .chain(iter::repeat(0).take(r_len - l_len)),
            ) {
                let (res, c) = l.overflowing_add(r);
                if carry {
                    let (c_res, d) = res.overflowing_add(1);
                    carry = c || d;
                    *l = c_res;
                } else {
                    *l = res;
                    carry = c;
                }
            }
            if carry {
                rhs.digits.push(1)
            }
            rhs
        }

    }

}

#[test]
fn add_test_1() {
    let a = 0u32.wrapping_sub(1);
    let a_big = BigInt::from(a);
    let b_big = BigInt::from(a);
    let c_big = a_big + b_big;

    let z = 0u32.wrapping_sub(2);

    let c_fixed = BigInt {
        sign: Sign::Positive,
        digits: vec![z, 1],
    };

    assert_eq!(c_big, c_fixed);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn scalar_mul_test_1() {
    let y: u32 = 915327;

    let a = BigInt{sign: Sign::Positive, digits: vec![3059078384, 2360247638, 2634550291, 6]};
    let b = BigInt{sign: Sign::Positive, digits: vec![356004624, 4070707789, 1201864523, 6053427]};

    assert_eq!(a * y, b);
}