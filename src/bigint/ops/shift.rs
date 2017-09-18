use bigint::BigInt;
use bigint::digit::{dbd_from_lo_hi, lo_hi_digits, DoubleBigDigit};
use bigint::digit::constants::*;

use std::ops::{Shl, ShlAssign, Shr, ShrAssign};
use std::iter;

impl ShlAssign<usize> for BigInt {
    fn shl_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }
        let (digit_shift, bit_shift) = (rhs / DIGIT_SIZE, rhs % DIGIT_SIZE);
        // TODO: Update with .insert_slice() when that exists.
        let extend_size = digit_shift + 1;
        self.digits.extend(iter::repeat(0).take(extend_size));

        // This is safe because we never reach past our own vector length,
        // thanks to the above extend call.
        unsafe {
            let ds = self.digits.as_mut_ptr();
            for i in (0..self.digits.len()).rev().skip(extend_size) {
                let v = ds.offset(i as isize);
                let [lo, hi] = lo_hi_digits((*v as DoubleBigDigit) << bit_shift);
                *v = 0;
                *ds.offset((i + digit_shift + 1) as isize) |= hi;
                *ds.offset((i + digit_shift) as isize) = lo;
            }
        }

        self.trim();
    }
}

impl Shl<usize> for BigInt {
    type Output = Self;
    fn shl(mut self, rhs: usize) -> Self::Output {
        self <<= rhs;
        self
    }
}

impl<'a> Shl<usize> for &'a BigInt {
    type Output = BigInt;
    fn shl(self, rhs: usize) -> Self::Output {
        let mut z = self.clone();
        z <<= rhs;
        z
    }
}

impl ShrAssign<usize> for BigInt {
    fn shr_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }
        let (digit_shift, bit_shift) = (rhs / DIGIT_SIZE, rhs % DIGIT_SIZE);

        unsafe {
            let ds = self.digits.as_mut_ptr();
            for i in (0..self.digits.len()).skip(digit_shift) {
                let v = ds.offset(i as isize);
                let shifted = dbd_from_lo_hi([0, *v]) >> bit_shift;
                let [lo, hi] = lo_hi_digits(shifted);
                *v = 0;
                *ds.offset((i.saturating_sub(digit_shift).saturating_sub(1)) as isize) |= lo;
                *ds.offset((i.saturating_sub(digit_shift)) as isize) = hi;
            }
            self.trim();
        }
    }
}

impl Shr<usize> for BigInt {
    type Output = Self;
    fn shr(mut self, rhs: usize) -> Self::Output {
        self >>= rhs;
        self
    }
}

impl<'a> Shr<usize> for &'a BigInt {
    type Output = BigInt;
    fn shr(self, rhs: usize) -> Self::Output {
        let mut z = self.clone();
        z >>= rhs;
        z
    }
}

#[test]
fn shl_equivalency_test() {
    use bigint::BigInt;
    use bigint::digit::BigDigit;
    use std::str::FromStr;
    let q = BigInt::from_str("49744649234615701185995702667471447085682723150956").unwrap();

    for t in 0..30usize {
        let z = q.clone();
        let y = q.clone();
        let q: BigDigit = 2;
        assert_eq!(z * q.pow(t as u32), y << t);
    }
}
