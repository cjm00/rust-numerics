#![allow(dead_code)]

mod arithmetic;
mod convert;
mod digit;
mod errors;
mod sign;

use rchunks::RChunks;

use self::digit::{ BigDigit, DoubleBigDigit};
use self::digit::constants::BASE_10_PARSE_CHUNK_SIZE;
use self::errors::BigIntParseError;
use self::sign::Sign;

use std::str::{self, FromStr};
use std::ascii::AsciiExt;
use std::cmp::{Ord, Ordering, PartialOrd};


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BigInt {
    sign: Sign,
    digits: Vec<BigDigit>,
}


impl BigInt {
    pub fn from_str_radix<S: AsRef<str>>(s: S, radix: usize) -> Result<Self, BigIntParseError> {
        use self::errors::BigIntParseError::*;

        let s = s.as_ref();

        match radix {
            10 => if !s.is_ascii_digit() {
                return Err(InvalidCharacters);
            },
            _ => unimplemented!(),
        }

        let radix_array: Vec<BigDigit> = match radix {
            10 => s
                .as_bytes()
                .rchunks(BASE_10_PARSE_CHUNK_SIZE)
                .map(|c| BigDigit::from_str(str::from_utf8(c).unwrap()).unwrap())
                .collect(),
            _ => unimplemented!(),
        };

        let mut radix_array_iter = radix_array.iter().rev().cloned();
        let first = radix_array_iter.next().unwrap();
        let mut output = BigInt::from(first);

        for r in radix_array_iter {
            output = (output * (radix as u32).pow(BASE_10_PARSE_CHUNK_SIZE as u32)) + r;
        }

        Ok(output)
    }

    pub fn is_zero(&self) -> bool {
        match self.sign {
            Sign::Zero => true,
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

    pub(crate) fn lo_hi_digits(d: DoubleBigDigit) -> [BigDigit; 2] {
        unsafe { ::std::mem::transmute(d) }
    }

    fn trim(&mut self) {
        loop {
            match self.digits.last() {
                Some(&0) => {
                    self.digits.pop();
                }
                _ => break,
            }
        }

        if self.digits.is_empty() {
            self.sign = Sign::Zero
        }
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
    use self::digit::DIGIT_SIZE;
    let mut a: DoubleBigDigit = 2;
    a = a.pow(DIGIT_SIZE as u32 + 2);

    assert_eq!([0, 4], BigInt::lo_hi_digits(a));
}

#[test]
fn trim_test() {
    let mut z = BigInt {
        sign: Sign::Positive,
        digits: vec![0; 50],
    };
    z.trim();
    assert!(z.is_zero());
}

#[test]
fn from_str_radix_test_1() {
    let s = "22209053970854587616243584284722270";
    let a = BigInt {
        sign: Sign::Positive,
        digits: vec![779322462, 594349670, 2880689586, 280317],
    };
    let b = BigInt::from_str_radix(s, 10).unwrap();
    assert_eq!(a, b);
}

#[test]
fn from_str_radix_test_2() {
    let s = "-22209053970854587616243584284722270";
    let a = BigInt {
        sign: Sign::Positive,
        digits: vec![779322462, 594349670, 2880689586, 280317],
    };
    let b = BigInt::from_str_radix(s, 10).unwrap();
    assert_eq!(a, b);
}
