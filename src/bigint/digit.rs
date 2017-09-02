pub use self::digit::{BigDigit, DoubleBigDigit, DIGIT_SIZE, BASE_10_PARSE_CHUNK_SIZE};


#[cfg(thicc_ints)]
mod digit {
    pub type BigDigit = u64;
    pub type DoubleBigDigit = u128;
    pub const DIGIT_SIZE: usize = 64;
    pub const BASE_10_PARSE_CHUNK_SIZE: usize = 16;
}

#[cfg(all(target_pointer_width = "64", not(thicc_ints)))]
mod digit {
    pub type BigDigit = u32;
    pub type DoubleBigDigit = u64;
    pub const DIGIT_SIZE: usize = 32;
    pub const BASE_10_PARSE_CHUNK_SIZE: usize = 8;
}

#[cfg(all(target_pointer_width = "32", not(thicc_ints)))]
mod digit {
    pub type BigDigit = u16;
    pub type DoubleBigDigit = u32;
    pub const DIGIT_SIZE: usize = 16;
    pub const BASE_10_PARSE_CHUNK_SIZE: usize = 4;
}