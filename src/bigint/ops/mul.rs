use bigint::{BigDigit, BigInt, DoubleBigDigit};
use bigint::Sign::*;

use bigint::digit::to_lo_hi;
use bigint::ops::add::sadd;
use bigint::ops::sub::{ssub_sign, ssub};

use std::ops::Mul;
use std::iter::repeat;

impl Mul<BigInt> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> Self::Output { naive_mul(&self, &rhs) }
}

impl<'a, 'b> Mul<&'a BigInt> for &'b BigInt {
    type Output = BigInt;
    fn mul(self, rhs: &'a BigInt) -> Self::Output { naive_mul(self, rhs) }
}

impl Mul<BigDigit> for BigInt {
    type Output = BigInt;
    fn mul(mut self, rhs: BigDigit) -> Self::Output {
        if self.is_zero() || rhs == 0 {
            return BigInt::zero();
        }

        let carry = dmul(&mut self.digits, rhs);

        if carry != 0 {
            self.digits.push(carry);
        }

        self
    }
}


pub(crate) fn naive_mul(lhs: &BigInt, rhs: &BigInt) -> BigInt {
    let sign = lhs.sign * rhs.sign;
    if sign == Zero {
        return BigInt::zero();
    }

    let mut digits = vec![0; lhs.digits.len() + rhs.digits.len()];

    mul3(&mut digits, &lhs.digits, &rhs.digits);

    let out = BigInt { sign, digits };
    out.trimmed()
}

/// 3 argument multiplication: `target += b * c` using different methods depending on argument lengths.
pub(crate) fn mul3(target: &mut [BigDigit], b: &[BigDigit], c: &[BigDigit]) {
    let (x, y) = if b.len() < c.len() { (b, c) } else { (c, b) };

    if x.len() <= 16 {
        n_mul3(target, x, y);
    } else {
        k_mul3(target, x, y);
    }
}

// Cribbed from num-bigint! https://github.com/rust-num/num-bigint/blob/master/src/algorithms.rs
pub(crate) fn k_mul3(target: &mut [BigDigit], x: &[BigDigit], y: &[BigDigit]) {
    debug_assert!(target.len() >= x.len() + y.len());

    let b = x.len() / 2;
    let (x0, x1) = x.split_at(b);
    let (y0, y1) = y.split_at(b);

    let s_len = x1.len() + y1.len() + 1;
    let mut scratch = BigInt{sign: Positive, digits: vec![0; s_len]};

    mul3(&mut scratch.digits, x1, y1);
    scratch.trim();

    sadd(&mut target[b..], &scratch.digits);
    sadd(&mut target[b * 2..], &scratch.digits);

    scratch.digits.truncate(0);
    scratch.digits.extend(repeat(0).take(s_len));

    mul3(&mut scratch.digits, x0, y0);
    scratch.trim();

    sadd(&mut target[..], &scratch.digits);
    sadd(&mut target[b..], &scratch.digits);

    let (j0_sign, j0) = ssub_sign(x1, x0);
    let (j1_sign, j1) = ssub_sign(y1, y0);

    match j0_sign * j1_sign {
        Positive => {
            scratch.digits.truncate(0);
            scratch.digits.extend(repeat(0).take(s_len));

            mul3(&mut scratch.digits, &j0, &j1);
            scratch.trim();
            ssub(&mut target[b..], &scratch.digits);
        },
        Negative => {
            mul3(&mut target[b..], &j0, &j1);
        },
        Zero => (),
    }
}

/// 3 argument naive multiplication: `target += b * c`
pub(crate) fn n_mul3(target: &mut [BigDigit], b: &[BigDigit], c: &[BigDigit]) {
    debug_assert!(target.len() >= b.len() + c.len());

    let mut carry: BigDigit = 0;

    for (i, l) in b.iter().cloned().enumerate() {
        if l == 0 {
            continue;
        }
        for (j, r) in c.iter().cloned().enumerate() {
            let [lo, hi] = to_lo_hi(
                l as DoubleBigDigit * r as DoubleBigDigit + target[i + j] as DoubleBigDigit +
                    carry as DoubleBigDigit,
            );
            target[i + j] = lo;
            if j + 1 != c.len() {
                carry = hi;
            } else {
                carry = 0;
                target[i + j + 1] = hi;
            }
        }
    }
}

/// Multiplies a slice by a single BigDigit, returning the carry.
pub(crate) fn dmul(lhs: &mut [BigDigit], rhs: BigDigit) -> BigDigit {
    let rhs = rhs as DoubleBigDigit;
    let mut carry: BigDigit = 0;
    for d in lhs.iter_mut() {
        let [lo, hi] = to_lo_hi((*d as DoubleBigDigit * rhs) + carry as DoubleBigDigit);
        *d = lo;
        carry = hi;
    }
    carry
}



#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(target_pointer_width = "64", not(feature = "thicc_ints")))]
    #[test]
    fn scalar_mul_test_1() {
        use bigint::sign::Sign;

        let y: u32 = 915327;

        let a = BigInt {
            sign: Sign::Positive,
            digits: vec![3059078384, 2360247638, 2634550291, 6],
        };
        let b = BigInt {
            sign: Sign::Positive,
            digits: vec![356004624, 4070707789, 1201864523, 6053427],
        };

        assert_eq!(a * y, b);
    }

    #[test]
    fn karatsuba_mul_coherence_test() {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        let foo: Vec<BigDigit> = rng.gen_iter().take(193).collect();
        let bar: Vec<BigDigit> = rng.gen_iter().take(101).collect();
        let mut k_output: Vec<BigDigit> = vec![0; foo.len() + bar.len() + 1];
        let mut n_output: Vec<BigDigit> = vec![0; foo.len() + bar.len() + 1];

        n_mul3(&mut n_output[..], &bar, &foo);
        k_mul3(&mut k_output[..], &bar, &foo);

        assert_eq!(n_output, k_output);
    }
}