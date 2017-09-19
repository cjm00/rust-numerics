use bigint::BigInt;
use bigint::digit::BigDigit;

use bigint::ops::div::short_divmod;

use std::ops::{Rem};

impl Rem<BigDigit> for BigInt {
    type Output = BigDigit;
    fn rem(self, rhs: BigDigit) -> Self::Output {
        short_divmod(&self, rhs, true).1.unwrap()
    }
}

impl<'a> Rem<BigDigit> for &'a BigInt {
    type Output = BigDigit;
    fn rem(self, rhs: BigDigit) -> Self::Output {
        short_divmod(self, rhs, true).1.unwrap()
    }
}