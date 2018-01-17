pub use self::digit::{chunk_size_from_radix, constants, BigDigit, DoubleBigDigit};


#[cfg(feature = "thicc_ints")]
mod digit {

    pub type BigDigit = u64;
    pub type DoubleBigDigit = u128;

    pub fn chunk_size_from_radix(radix: u32) -> Option<usize> {
        use self::constants::*;
        match radix {
            2 => Some(BASE_2_PARSE_CHUNK_SIZE),
            8 => Some(BASE_8_PARSE_CHUNK_SIZE),
            10 => Some(BASE_10_PARSE_CHUNK_SIZE),
            16 => Some(BASE_16_PARSE_CHUNK_SIZE),
            _ => None,
        }
    }

    pub mod constants {
        pub const DIGIT_MAX: u64 = ::std::u64::MAX;
        pub const DIGIT_SIZE: usize = 64;

        pub const BASE_2_PARSE_CHUNK_SIZE: usize = 62;
        pub const BASE_8_PARSE_CHUNK_SIZE: usize = 20;
        pub const BASE_10_PARSE_CHUNK_SIZE: usize = 16;
        pub const BASE_16_PARSE_CHUNK_SIZE: usize = 16;

        pub const BASE_10_FORMAT_CHUNK_SIZE: u64 = 1_000_000_000_000_000_000_0;
        pub const BASE_10_FORMAT_PAD_SIZE: usize = 19;
    }

}

#[cfg(all(target_pointer_width = "64", not(feature = "thicc_ints")))]
mod digit {

    pub type BigDigit = u32;
    pub type DoubleBigDigit = u64;

    pub fn chunk_size_from_radix(radix: u32) -> Option<usize> {
        use self::constants::*;
        match radix {
            2 => Some(BASE_2_PARSE_CHUNK_SIZE),
            8 => Some(BASE_8_PARSE_CHUNK_SIZE),
            10 => Some(BASE_10_PARSE_CHUNK_SIZE),
            16 => Some(BASE_16_PARSE_CHUNK_SIZE),
            _ => None,
        }
    }

    pub mod constants {
        pub const DIGIT_MAX: u32 = ::std::u32::MAX;
        pub const DIGIT_SIZE: usize = 32;

        pub const BASE_2_PARSE_CHUNK_SIZE: usize = 31;
        pub const BASE_8_PARSE_CHUNK_SIZE: usize = 10;
        pub const BASE_10_PARSE_CHUNK_SIZE: usize = 8;
        pub const BASE_16_PARSE_CHUNK_SIZE: usize = 8;

        pub const BASE_10_FORMAT_CHUNK_SIZE: u32 = 1_000_000_000;
        pub const BASE_10_FORMAT_PAD_SIZE: usize = 9;

    }
}

#[cfg(all(target_pointer_width = "32", not(feature = "thicc_ints")))]
mod digit {

    pub type BigDigit = u16;
    pub type DoubleBigDigit = u32;

    pub mod constants {
        pub const DIGIT_MAX: u16 = ::std::u16::MAX;
        pub const DIGIT_SIZE: usize = 16;
        pub const BASE_10_PARSE_CHUNK_SIZE: usize = 4;
    }
}

#[inline]
pub(crate) fn to_lo_hi(d: DoubleBigDigit) -> [BigDigit; 2] {
    use self::constants::DIGIT_SIZE;

    [ d as BigDigit, (d >> DIGIT_SIZE) as BigDigit ]
}

#[inline]
pub(crate) fn from_lo_hi(lh: [BigDigit; 2]) -> DoubleBigDigit {
    use self::constants::DIGIT_SIZE;

    (lh[0] as DoubleBigDigit) + ((lh[1] as DoubleBigDigit) << DIGIT_SIZE)
}


#[test]
fn lo_hi_digit_test() {
    use bigint::digit::constants::DIGIT_SIZE;
    use bigint::digit::to_lo_hi;
    let mut a: DoubleBigDigit = 2;
    a = a.pow(DIGIT_SIZE as u32 + 2);

    assert_eq!([0, 4], to_lo_hi(a));
}
