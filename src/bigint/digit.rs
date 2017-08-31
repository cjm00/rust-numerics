pub use self::digit::{BigDigit, DoubleBigDigit, DIGIT_SIZE};


#[cfg(target_arch = "x86_64")]
mod digit {
    pub type BigDigit = u32;
    pub type DoubleBigDigit = u64;
    pub const DIGIT_SIZE: usize = 32;
}

#[cfg(target_arch = "x86")]
mod digit {
    pub type BigDigit = u16;
    pub type DoubleBigDigit = u32;
    pub const DIGIT_SIZE: usize = 16;
}