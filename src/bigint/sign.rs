use std::ops::{Mul, Neg};
use std::cmp::{PartialOrd, Ord, Ordering};

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
            (Zero, _) | (_, Zero) => Zero,
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
            Negative => Positive,
            Positive => Negative,
        }
    }
}

impl PartialOrd<Sign> for Sign {
    fn partial_cmp(&self, rhs: &Sign) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Sign {
    fn cmp(&self, other: &Self) -> Ordering {
        use self::Ordering::*;
        use self::Sign::*;
        match (*self, *other) {
            (Positive, Positive) => Equal,
            (Zero, Zero) => Equal,
            (Negative, Negative) => Equal,
            (Positive, _) => Greater,
            (_, Positive) => Less,
            (Negative, _) => Less,
            (_, Negative) => Greater,
        }
    }
}