use bigint::BigInt;
use bigint::sign::Sign;
use bigint::digit::chunk_size_from_radix;
use bigint::digit::BigDigit;
use bigint::errors::BigIntParseError;

use nom::IResult::*;
use rchunks::RChunks;

use std::str::{self, FromStr};

static BASE_2_CHARACTERS: &str = "01";
static BASE_8_CHARACTERS: &str = "01234567";
static BASE_10_CHARACTERS: &str = "0123456789";
static BASE_16_CHARACTERS: &str = "0123456789abcdef";


impl FromStr for BigInt {
    type Err = BigIntParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use super::BigIntParseError::*;
        if s.is_empty() {return Err(EmptyInput)}

        let builder = BigIntBuilder::parse_from_str(&s)?;
        Ok(builder.to_bigint())
        
    }
}

pub(crate) struct BigIntBuilder<'a> {
    sign: Sign,
    radix: u32,
    digit_str: &'a str,
}

impl<'a> BigIntBuilder<'a> {
    pub fn parse_from_str<'q, S: AsRef<str>>(
        input: &'q S,
    ) -> Result<BigIntBuilder<'q>, BigIntParseError> {
        let input = input.as_ref();
        match parse_bigint(input) {
            Done(_, u) => Ok(u),
            Incomplete(_) => Err(BigIntParseError::EmptyInput),
            Error(_) => Err(BigIntParseError::Unknown),
        }
    }

    pub fn to_bigint(self) -> BigInt {
        let sign = self.sign;
        let radix = self.radix;

        let radix_vec: Vec<BigDigit> = self.digit_str
            .as_bytes()
            .rchunks(chunk_size_from_radix(radix).unwrap())
            .map(|c| BigDigit::from_str(str::from_utf8(c).unwrap()).unwrap())
            .collect();

        let mut radix_vec_iter = radix_vec.into_iter().rev();
        let first = radix_vec_iter.next().unwrap();
        let mut output = BigInt::from(first);
        output.sign = sign;

        for r in radix_vec_iter {
            output = (output * radix.pow(chunk_size_from_radix(radix).unwrap() as u32)) + r;
        }
        output
    }
}

named!(parse_bigint<&str, BigIntBuilder>,
    do_parse!(
        sign: determine_sign >>
        radix: determine_radix >>
        digit_str: switch!(value!(radix),
            2 => is_a_s!(BASE_2_CHARACTERS) |
            8 => is_a_s!(BASE_8_CHARACTERS) |
            10 => is_a_s!(BASE_10_CHARACTERS) |
            16 => is_a_s!(BASE_16_CHARACTERS)
            ) >>
        (BigIntBuilder{sign, radix, digit_str})

));


named!(determine_sign<&str, Sign>,
    alt_complete!(
        value!(Sign::Negative, tag!("-")) |
        value!(Sign::Positive)
    )
);

named!(determine_radix<&str, u32>,
    alt_complete!(hex | octal | binary | decimal)
);

named!(hex<&str, u32>,
do_parse!(
    tag!("0x") >>
    (16)
));

named!(octal<&str, u32>,
    do_parse!(
        tag!("0o") >>
        (8)
));

named!(binary<&str, u32>,
    do_parse!(
        tag!("0b") >>
        (2)
));

named!(decimal<&str, u32>,
    value!(10)
);
