#![allow(dead_code)]

mod arithmetic;
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


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BigInt {
    sign: Sign,
    digits: Vec<BigDigit>,
}


impl BigInt {
    pub fn is_zero(&self) -> bool {
        match self.sign {
            Sign::Zero => true,
            _ => false,
        }
    }
    
    pub fn is_positive(&self) -> bool {
        match self.sign {
            Sign::Positive => true,
            _ => false,
        }
    }

    pub fn is_negative(&self) -> bool {
        match self.sign {
            Sign::Negative => true,
            _ => false,
        }
    }

    pub fn zero() -> Self {
        BigInt {
            sign: Sign::Zero,
            digits: vec![],
        }
    }

    pub fn negate(&mut self) {
        self.sign = -self.sign;
    }

    fn trim(&mut self) {
        loop {
            match self.digits.last() {
                Some(&0) => self.digits.pop(),
                _ => break,
            };
        }

        if self.digits.is_empty() {
            self.sign = Sign::Zero
        }
    }
}

impl Neg for BigInt {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.sign = -self.sign;
        self
    }
}

impl PartialOrd for BigInt {
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

/*
impl FromStr for BigInt {
    type Err = BigIntParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}
*/

#[test]
fn lo_hi_digit_test() {
    use bigint::digit::constants::DIGIT_SIZE;
    let mut a: DoubleBigDigit = 2;
    a = a.pow(DIGIT_SIZE as u32 + 2);

    assert_eq!([0, 4], BigInt::lo_hi_digits(a));
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
