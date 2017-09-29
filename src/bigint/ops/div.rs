use bigint::BigInt;
use bigint::digit::{BigDigit, DoubleBigDigit, dbd_from_lo_hi};
use bigint::digit::constants::DIGIT_MAX;
use bigint::sign::Sign;

use bigint::ops::mul::smul;
use bigint::ops::add::sadd;
use bigint::ops::sub::ssub_digit;

use std::cmp::Ordering::*;
use std::ops::{ShlAssign, Div};

impl Div<BigDigit> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: BigDigit) -> Self::Output {
        short_divmod(&self, rhs, false).0
    }
}

impl<'a> Div<BigDigit> for &'a BigInt {
    type Output = BigInt;
    fn div(self, rhs: BigDigit) -> Self::Output {
        short_divmod(self, rhs, false).0
    }
}



pub(crate) fn short_divmod(dividend: &BigInt, divisor: BigDigit, return_remainder: bool) -> (BigInt, Option<BigDigit>) {
    assert!(divisor != 0, "Can't divide by zero");
    if dividend.is_zero() {
        if return_remainder {
            return (BigInt::zero(), Some(0))
        } else {
            return (BigInt::zero(), None)
        }
    }

    let divisor = divisor as DoubleBigDigit;
    let mut quo = vec![0; dividend.digits.len()];
    let mut carry: BigDigit = 0;

    for (d, q) in dividend.digits.iter().cloned().zip(quo.iter_mut()).rev() {
        let res = (dbd_from_lo_hi([d, carry]) / divisor) as BigDigit;
        let rem = (dbd_from_lo_hi([d, carry]) % divisor) as BigDigit;
        *q = res;
        carry = rem;
    }

    if return_remainder {
        (BigInt::from_vec(quo), Some(carry))
    } else {
        (BigInt::from_vec(quo), None)
    }
}

/// Returns (dividend / divisor, remainder). This algorithm taken from TAOCP 4.3.1
pub(crate) fn divmod(
    mut dividend: BigInt,
    mut divisor: BigInt,
    return_remainder: bool,
) -> (BigInt, Option<BigInt>) {
    assert!(!divisor.is_zero(), "Can't divide by zero");
    if dividend.is_zero() {
        if return_remainder {
            return (BigInt::zero(), Some(BigInt::zero()));
        } else {
            return (BigInt::zero(), None);
        }
    }
    let cmp = dividend.cmp(&divisor);
    match cmp {
        Equal => return (BigInt::one(), Some(BigInt::zero())),
        Less => return (BigInt::zero(), Some(dividend.clone())),
        Greater => (),
    }

    let shift_size = normalization_shift_size(&divisor) as usize;

    // TODO: https://github.com/rust-lang/rust/issues/25753
    dividend.shl_assign(shift_size);
    divisor.shl_assign(shift_size);

    let mut quotient: Vec<BigDigit>;
    let mut scratch: Vec<BigDigit> = vec![0; divisor.digits.len() + 1];
    {
        // Constants for the division loop.
        let m = dividend.digits.len() - divisor.digits.len() - 1;
        let n = divisor.digits.len();
        let u = &mut dividend.digits;
        let v = &mut divisor.digits;
        let b = DIGIT_MAX as DoubleBigDigit + 1;

        quotient = vec![0; m + 1];

        let mut j = m;

        while j != usize::max_value() {
            let trial = (u[j + n] as DoubleBigDigit * b) + u[j + n - 1] as DoubleBigDigit;
            let mut qhat = trial / (v[n - 1] as DoubleBigDigit) + 1;
            let mut rhat = trial % (v[n - 1] as DoubleBigDigit);

            loop {
                if (qhat == b) ||
                    (qhat * v[n - 2] as DoubleBigDigit >
                        (rhat * b) + u[j + n - 2] as DoubleBigDigit)
                {
                    qhat -= 1;
                    rhat += v[n - 1] as DoubleBigDigit;
                    if rhat < b {
                        continue;
                    }
                }
                break;
            }

            quotient[j] = qhat as BigDigit;
            scratch[..n].copy_from_slice(v);
            scratch[n] = 0;
            let borrow = ssub_with_mul(&mut u[j..j + n + 1], &mut scratch, qhat as BigDigit);


            if borrow {
                ssub_digit(&mut quotient[j..], 1);
                scratch[..n].copy_from_slice(v);
                scratch[n] = 0;
                let carry = sadd(&mut u[j..j + n], &scratch);
                debug_assert_eq!(carry, true);
            }
            j = j.wrapping_sub(1);
        }
    }

    let quo = BigInt {
        sign: Sign::Positive,
        digits: quotient,
    };
    if return_remainder {
        dividend.digits.truncate(divisor.digits.len());
        (quo, Some(dividend.trimmed() >> shift_size))
    } else {
        (quo, None)
    }
}

/// Sets dividend to dividend - q * divisor. If dividend is negative, it is left as the b's
/// complement, where b is the radix of BigDigit.
fn ssub_with_mul(dividend: &mut [BigDigit], divisor: &mut [BigDigit], q: BigDigit) -> bool {
    let _carry = smul(divisor, q);
    debug_assert_eq!(_carry, 0);

    let mut carry = false;

    assert_eq!(dividend.len(), divisor.len());
    for (l, r) in dividend.iter_mut().zip(divisor.iter().cloned()) {
        let (res, c) = l.overflowing_sub(r);
        let (res, d) = if carry {
            res.overflowing_sub(1)
        } else {
            (res, false)
        };
        *l = res;
        carry = c || d;
    }

    carry
}

fn normalization_shift_size(input: &BigInt) -> u32 {
    input.digits.last().unwrap().leading_zeros()
}

#[test]
fn normalization_test() {
    use bigint::digit::constants::DIGIT_MAX;
    use bigint::digit::BigDigit;
    use bigint::BigInt;

    let t: BigDigit = 9274;
    let s = BigInt::from(t);

    let size = normalization_shift_size(&s) as usize;
    let s = s << size;

    assert!(*s.digits.last().unwrap() > (DIGIT_MAX / 2));
}

#[test]
fn short_divmod_test() {
    use std::str::FromStr;
    let dividend = BigInt::from_str("159227301757406318958308608461596464563224530763743").unwrap();
    let divisor: BigDigit = 74495;
    let quotient = BigInt::from_str("2137422669406085226636802583550526405305383324").unwrap();
    let remainder: BigDigit = 42363;

    let res = short_divmod(&dividend, divisor, true);
    assert_eq!(res, (quotient, Some(remainder)));
}