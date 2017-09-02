use std::ops::{Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sign {
    Positive,
    Negative,
    Zero,
}

impl Mul<Sign> for Sign {
    type Output = Sign;
    #[inline]
    fn mul(self, rhs: Sign) -> Self::Output {
        use self::Sign::*;
        match (self, rhs) {
            (Zero, _) => Zero,
            (_, Zero) => Zero,
            (Positive, Positive) => Positive,
            (Negative, Negative) => Positive,
            (Positive, Negative) => Negative,
            (Negative, Positive) => Negative,
        }
    }
}

impl Neg for Sign {
    type Output = Sign;
    #[inline]
    fn neg(self) -> Self::Output {
        use self::Sign::*;
        match self {
            Zero => Zero,
            Negative => Negative,
            Positive => Positive,
        }
    }
}
