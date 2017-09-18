use std::fmt::{Binary, Error, Formatter};

use bigint::BigInt;
use bigint::sign::Sign::*;
use bigint::digit::constants::DIGIT_SIZE;


impl Binary for BigInt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.sign {
            Positive => {
                if f.sign_plus() {
                    write!(f, "+")?
                }
            }
            Negative => write!(f, "-")?,
            Zero => {}
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

#[test]
fn binary_format_test_1() {
    use std::str::FromStr;
    let s = "0b1001010101011011101101010101";
    let s_int = BigInt::from_str(s).unwrap();
    let s_to_str = format!("{:#b}", s_int);
    assert_eq!(s_to_str, s);
}
