use std::convert::From;

use bigint::{Sign, BigInt};
use bigint::digit::BigDigit;



macro_rules! impl_from_unsigned {
    ($($x:ty),*) => { $(
        impl From<$x> for BigInt {
            fn from(src: $x) -> BigInt {
                BigInt {sign: Sign::Positive, digits: vec![src as BigDigit]}
            }
        }
        )*
    }
}

impl_from_unsigned!(u8, u16, u32);

