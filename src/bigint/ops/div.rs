use bigint::BigInt;
use bigint::digit::{BigDigit, DoubleBigDigit};
use bigint::digit::constants::DIGIT_MAX;
use bigint::sign::Sign;

use bigint::ops::mul::smul;
use bigint::ops::add::sadd;
use bigint::ops::sub::ssub_digit;

use std::cmp::Ordering::*;
use std::ops::{ShlAssign};


/// Returns (lhs / rhs, remainder). This algorithm taken from TAOCP 4.3.1
pub(crate) fn divmod(mut lhs: BigInt, mut rhs: BigInt) -> (BigInt, BigInt) {
    assert!(!rhs.is_zero(), "Can't divide by zero");
    if lhs.is_zero() {
        return (BigInt::zero(), BigInt::zero());
    }
    let cmp = lhs.cmp(&rhs);
    match cmp {
        Equal => return (BigInt::one(), BigInt::zero()),
        Less => return (BigInt::zero(), lhs.clone()),
        Greater => (),
    }

    let shift_size = normalization_shift_size(&rhs) as usize;

    // TODO: https://github.com/rust-lang/rust/issues/25753
    lhs.shl_assign(shift_size);
    rhs.shl_assign(shift_size);

    let mut quotient: Vec<BigDigit>;

    {
    // Constants for the division loop.
    let m = lhs.digits.len() - rhs.digits.len() - 1;
    let n = rhs.digits.len();
    let u = &mut lhs.digits;
    let v = &mut rhs.digits;
    let b = DIGIT_MAX as DoubleBigDigit + 1;

    quotient = vec![0; m + 1];

    let mut j = m;

    while j != usize::max_value() {
        let trial = (u[j+n] as DoubleBigDigit * b) + u[j+n-1] as DoubleBigDigit;
        let mut qhat = trial / (v[n - 1] as DoubleBigDigit) + 1;
        let mut rhat = trial % (v[n - 1] as DoubleBigDigit);

        loop {
            if (qhat == b) ||
                (qhat * v[n - 2] as DoubleBigDigit > (rhat * b) + u[j+n-2] as DoubleBigDigit)
            {
                qhat -= 1;
                rhat += v[n - 1] as DoubleBigDigit;
                if rhat < b {continue}
            }
            break;
        }
        
        quotient[j] = qhat as BigDigit;
        let borrow = ssub_with_mul(&mut u[j..j+n+1], &v[..], qhat as BigDigit);
   

        if borrow {
            ssub_digit(&mut quotient[j..], 1);
            let mut v_ex = v.to_owned();
            v_ex.push(0);
            let carry = sadd(&mut u[j..j+n], &v_ex[..]);
            debug_assert_eq!(carry, true);
        }
        j = j.wrapping_sub(1);
    }
   }

    let quo = BigInt{sign: Sign::Positive, digits: quotient};
    lhs.digits.truncate(rhs.digits.len());
    (quo, lhs.trimmed() >> shift_size)
}

/// Sets lhs to lhs - q * rhs. If LHS is negative, it is left as the b's complement, where b is the radix of BigDigit.
fn ssub_with_mul(lhs: &mut [BigDigit], rhs: &[BigDigit], q: BigDigit) -> bool {
    let mut rhs: Vec<BigDigit> = rhs.into();
    rhs.push(0);
    let _carry = smul(&mut rhs, q);
    debug_assert_eq!(_carry, 0);

    let mut carry = false;

    assert_eq!(lhs.len(), rhs.len());
    for (l, r) in lhs.iter_mut().zip(rhs.iter().cloned()) {
        let (res, c) = l.overflowing_sub(r);
        let (res, d) = if carry {res.overflowing_sub(1)} else {(res, false)};
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
