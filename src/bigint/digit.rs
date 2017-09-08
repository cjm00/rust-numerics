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
        pub const DIGIT_SIZE: usize = 64;
        pub const BASE_2_PARSE_CHUNK_SIZE: usize = 62;
        pub const BASE_8_PARSE_CHUNK_SIZE: usize = 20;
        pub const BASE_10_PARSE_CHUNK_SIZE: usize = 16;
        pub const BASE_16_PARSE_CHUNK_SIZE: usize = 16;
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

        pub const DIGIT_SIZE: usize = 32;
        pub const BASE_2_PARSE_CHUNK_SIZE: usize = 31;
        pub const BASE_8_PARSE_CHUNK_SIZE: usize = 10;
        pub const BASE_10_PARSE_CHUNK_SIZE: usize = 8;
        pub const BASE_16_PARSE_CHUNK_SIZE: usize = 8;

    }
}

#[cfg(all(target_pointer_width = "32", not(feature = "thicc_ints")))]
mod digit {

    pub type BigDigit = u16;
    pub type DoubleBigDigit = u32;

    pub mod constants {
        pub const DIGIT_SIZE: usize = 16;
        pub const BASE_10_PARSE_CHUNK_SIZE: usize = 4;
    }
}
