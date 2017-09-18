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

#[cfg(all(target_pointer_width = "64", not(feature = "thicc_ints")))]
impl From<u64> for BigInt {
    fn from(src: u64) -> Self {
        let [lo, hi]: [u32; 2] = unsafe { ::std::mem::transmute(src) };
        let out = BigInt {
            sign: Sign::Positive,
            digits: vec![lo, hi],
        };

        out.trimmed()
    }
}

#[cfg(feature = "thicc_ints")]
impl From<u64> for BigInt {
    fn from(src: u64) -> Self {
        BigInt {
            sign: Sign::Positive,
            digits: vec![src as BigDigit],
        }
    }
}
