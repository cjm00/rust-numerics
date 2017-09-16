//! A flexible and fast BigInteger implementation.

#![allow(dead_code)]

mod ops;
mod convert;
mod digit;
mod errors;
mod sign;
mod parse;
mod format;


use self::digit::{BigDigit, DoubleBigDigit};
use self::errors::BigIntParseError;
use self::sign::Sign;
use std::ops::Neg;

use std::cmp::{Ord, Ordering, PartialOrd};

/// An arbitrary size integer
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BigInt {
    sign: Sign,
    digits: Vec<BigDigit>,
}


impl BigInt {

    /// Returns true if the BigInt is zero and false otherwise;   
    #[inline]
    pub fn is_zero(&self) -> bool {
        match self.sign {
            Sign::Zero => true,
            _ => false,
        }
    }
    
    /// Returns true if the BigInt is strictly greater than zero.
    #[inline]
    pub fn is_positive(&self) -> bool {
        match self.sign {
            Sign::Positive => true,
            _ => false,
        }
    }
    
    /// Returns true if the BigInt is strictly less than zero.
    #[inline]
    pub fn is_negative(&self) -> bool {
        match self.sign {
            Sign::Negative => true,
            _ => false,
        }
    }

    /// Returns a BigInt with a value of zero.
    #[inline]
    pub fn zero() -> Self {
        BigInt::from(0u8)
    }

    /// Returns a BigInt with a value of positive 1.
    #[inline]
    pub fn one() -> Self {
        BigInt {
            sign: Sign::Positive,
            digits: vec![1],
        }
    }
    
    /// Changes self to have the opposite sign. No change if self is zero.
    #[inline]
    pub fn negate(&mut self) {
        self.sign = -self.sign;
    }

    #[inline]
    fn trim(&mut self) {
        while let Some(&0) = self.digits.last() {
            self.digits.pop();
        }

        if self.digits.is_empty() {
            self.sign = Sign::Zero
        }
    }

    /// Returns self with all leading zeroes removed.
    #[inline]
    fn trimmed(mut self) -> Self {
        self.trim();
        self
    }

    /// Extends this BigInt to have at least `size` digits. Does nothing if this already has at least `size` digits.     
    #[inline]
    fn grow_to_hold(&mut self, size: usize) {
        if size > self.digits.len() {
            self.digits.resize(size, 0);
        }
    }
}

impl Neg for BigInt {
    type Output = Self; 
    #[inline]
    fn neg(mut self) -> Self::Output {
        self.sign = -self.sign;
        self
    }
}

impl PartialOrd for BigInt { 
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        use self::Ordering::*;
        use self::Sign::*;

        let sgn = match (self.sign.cmp(&other.sign), self.sign) {
            (Greater, _) => return Greater,
            (Less, _) => return Less,
            (Equal, Zero) => return Equal,
            (Equal, Negative) => Negative,
            (Equal, Positive) => Positive,
        };

        if sgn == Positive {
            self.digits.len().cmp(&other.digits.len()).then_with(|| {
                for (s, o) in self.digits.iter().zip(other.digits.iter()).rev() {
                    match s.cmp(o) {
                        Greater => return Greater,
                        Less => return Less,
                        Equal => continue,
                    }
                }
                Equal
            })
        } else {
            self.digits
                .len()
                .cmp(&other.digits.len())
                .reverse()
                .then_with(|| {
                    for (s, o) in self.digits.iter().zip(other.digits.iter()).rev() {
                        match s.cmp(o) {
                            Greater => return Less,
                            Less => return Greater,
                            Equal => continue,
                        }
                    }
                    Equal
                })
        }
    }
}

#[test]
fn trim_test() {
    use bigint::sign::Sign;
    let mut z = BigInt {
        sign: Sign::Positive,
        digits: vec![0; 50],
    };
    z.trim();
    assert!(z.is_zero());
}

#[cfg(all(target_pointer_width = "64", not(feature = "thicc_ints")))]
#[test]
fn from_str_radix_test_1() {
    use bigint::sign::Sign;
    use std::str::FromStr;
    let s = "22209053970854587616243584284722270";
    let a = BigInt {
        sign: Sign::Positive,
        digits: vec![779322462, 594349670, 2880689586, 280317],
    };
    let b = BigInt::from_str(s).unwrap();
    assert_eq!(a, b);
}

#[cfg(all(target_pointer_width = "64", not(feature = "thicc_ints")))]
#[test]
fn from_str_radix_test_2() {
    use bigint::sign::Sign;
    use std::str::FromStr;
    let s = "-22209053970854587616243584284722270";
    let a = BigInt {
        sign: Sign::Negative,
        digits: vec![779322462, 594349670, 2880689586, 280317],
    };
    let b = BigInt::from_str(s).unwrap();
    assert_eq!(a, b);
}
