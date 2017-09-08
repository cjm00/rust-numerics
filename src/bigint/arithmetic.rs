use bigint::BigInt;
use bigint::Sign::*;
use bigint::digit::{BigDigit, DoubleBigDigit};

use std::ops::{Add, Mul, Neg, Sub};

impl Neg for BigInt {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.sign = -self.sign;
        self
    }
}


impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Positive, Positive) => add::strict_pos_overflow_add(self, rhs),
            (Positive, Negative) => self - (-rhs),
            (Zero, _) => rhs,
            (_, Zero) => self,
            (Negative, Positive) => rhs - self,
            (Negative, Negative) => -(-self + -rhs),
        }
    }
}


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
            (Negative, Negative) => {
                sub::naive_sub(&mut self, &rhs);
                self
            }
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

impl Mul<BigInt> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> Self::Output {
        mul::naive_mul(&self, &rhs)
    }
}

pub(crate) mod mul {

    use bigint::BigInt;
    use bigint::digit::DoubleBigDigit;
    use bigint::Sign::*;
    use super::add::ripple_add;

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


}

pub(crate) mod add {
    use bigint::BigInt;
    use bigint::digit::BigDigit;

    pub fn strict_pos_overflow_add(mut lhs: BigInt, mut rhs: BigInt) -> BigInt {
        if lhs.digits.len() >= rhs.digits.len() {
            lhs.digits.push(0);
            for (i, d) in rhs.digits.iter().cloned().enumerate() {
                ripple_add(&mut lhs.digits[i..], d);
            }
            lhs.trim();
            lhs
        } else {
            rhs.digits.push(0);
            for (i, d) in lhs.digits.iter().cloned().enumerate() {
                ripple_add(&mut rhs.digits[i..], d);
            }
            rhs.trim();
            rhs
        }
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

#[test]
fn add_test_1() {
    use bigint::sign::Sign;

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
