use std::fmt::{Binary, Display, Error, Formatter};

use bigint::BigInt;
use bigint::sign::Sign::*;
use bigint::digit::constants::*;


impl Binary for BigInt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.sign {
            Positive => {
                if f.sign_plus() {
                    write!(f, "+")?
                }
            }
            Negative => write!(f, "-")?,
            Zero => {
                write!(f, "0")?;
                return Ok(())
            }
        }

        if f.alternate() {
            write!(f, "0b")?
        }

        let mut i = self.digits.iter().rev();

        let first = i.next().unwrap();
        write!(f, "{:b}", first)?;

        for d in i {
            write!(f, "{:0>d_size$b}", d, d_size = DIGIT_SIZE)?;
        }

        Ok(())
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter) ->Result<(), Error> {
        match self.sign {
            Positive => {
                if f.sign_plus() {
                    write!(f, "+")?
                }
            }
            Negative => write!(f, "-")?,
            Zero => {
                write!(f, "0")?;
                return Ok(())
            }
        }

        let mut digits = vec![];
        let mut reduced = self.clone();

        while !reduced.is_zero() {
            digits.push(&reduced % BASE_10_FORMAT_CHUNK_SIZE);
            reduced = &reduced / BASE_10_FORMAT_CHUNK_SIZE;
        }

        let mut i = digits.into_iter().rev();

        write!(f, "{}", i.next().unwrap())?;

        for digit_chunk in i {
            write!(f, "{:0>d_size$}", digit_chunk, d_size = BASE_10_FORMAT_PAD_SIZE)?;
        }

        Ok(())
    }
}

#[test]
fn binary_format_test_1() {
    use std::str::FromStr;
    let s = "0b1001010101011011101101010101";
    let s_int = BigInt::from_str(s).unwrap();
    let s_to_str = format!("{:#b}", s_int);
    assert_eq!(s_to_str, s);
}

#[test]
fn display_involution_test() {
    use std::str::FromStr;
    let s = "9802347092384702938472093847234820923487";
    let s_toint = BigInt::from_str(s).unwrap();
    let s_toint_tostring = format!("{}", s_toint);
    assert_eq!(s, s_toint_tostring);
}
