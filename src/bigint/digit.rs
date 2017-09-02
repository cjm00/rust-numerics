pub use self::digit::{BigDigit, DoubleBigDigit, DIGIT_SIZE, BASE_10_PARSE_CHUNK_SIZE};


#[cfg(target_pointer_width = "64")]
mod digit {
    pub type BigDigit = u32;
    pub type DoubleBigDigit = u64;
    pub const DIGIT_SIZE: usize = 32;
    pub const BASE_10_PARSE_CHUNK_SIZE: usize = 8;
}

#[cfg(target_pointer_width = "32")]
mod digit {
    pub type BigDigit = u16;
    pub type DoubleBigDigit = u32;
    pub const DIGIT_SIZE: usize = 16;
    pub const BASE_10_PARSE_CHUNK_SIZE: usize = 4;
}