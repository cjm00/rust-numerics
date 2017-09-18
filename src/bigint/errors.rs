#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BigIntParseError {
    InvalidCharacters,
    EmptyInput,
    InvalidRadix,
    Unknown,
}
