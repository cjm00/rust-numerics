use bigint::{BigInt, BigDigit};
use bigint::Sign::*;

use std::ops::Add;


impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(mut self, rhs: BigInt) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Positive, Positive) => {
                self.grow_to_hold(rhs.digits.len());
                let carry = sadd(&mut self.digits, &rhs.digits);
                self.digits.push(carry as BigDigit);
                self.trimmed()
            }
            (Positive, Negative) => self - (-rhs),
            (Zero, _) => rhs,
            (_, Zero) => self,
            (Negative, Positive) => rhs - self,
            (Negative, Negative) => -(-self + -rhs),
        }
    }
}


impl<'a, 'b> Add<&'b BigInt> for &'a BigInt {
    type Output = BigInt;
    fn add(self, rhs: &'b BigInt) -> BigInt {
        if self >= rhs {
            let mut output = self.clone();
            add_and_grow(&mut output, rhs);
            output
        } else {
            let mut output = rhs.clone();
            add_and_grow(&mut output, self);
            output
        }
    }
}

impl Add<BigDigit> for BigInt {
    type Output = BigInt;
    fn add(mut self, rhs: BigDigit) -> Self::Output {
        if self.is_zero() {
            self.digits.push(rhs);
            return self;
        }
        let (res, mut carry) = self.digits[0].overflowing_add(rhs);
        self.digits[0] = res;

        let mut index = 1usize;
        while carry && (index < self.digits.len()) {
            let (r, c) = self.digits[index].overflowing_add(1);
            self.digits[index] = r;
            carry = c;
            index += 1;
        }

        if carry {
            self.digits.push(1);
        }

        self
    }
}

fn add_and_grow(lhs: &mut BigInt, rhs: &BigInt) {
    lhs.grow_to_hold(rhs.digits.len());
    let grow = sadd(&mut lhs.digits, &rhs.digits);
    if grow {lhs.digits.push(1)}
    lhs.trim()
}

pub(crate) fn sadd(lhs: &mut [BigDigit], rhs: &[BigDigit]) -> bool {
    let mut carry = false;
    for (l, r) in lhs.iter_mut().zip(rhs.iter().cloned()) {
        let (res, c) = l.overflowing_add(r);
        let (res, d) = if carry {res.overflowing_add(1)} else {(res, false)};
        *l = res;
        carry = c || d;
    }
    carry
}


#[test]
fn add_test_1() {
    use bigint::sign::Sign;

    let a: BigDigit = 0;
    let a = a.wrapping_sub(1);

    let a_big = BigInt::from(a);
    let b_big = BigInt::from(a);
    let c_big = a_big + b_big;

    let z: BigDigit = 0;
    let z = z.wrapping_sub(2);

    let c_fixed = BigInt {
        sign: Sign::Positive,
        digits: vec![z, 1],
    };

    assert_eq!(c_big, c_fixed);
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    extern crate test;
    
    #[bench]
    fn public_add_bench_100(z: &mut test::Bencher) {
        use std::str::FromStr;
        use ::bigint::BigInt;
        let a = BigInt::from_str("5605552357266437729280504134296683206097961396781567121352972852719206640545169925410820532634965117").unwrap();
        let b = BigInt::from_str("4247965229701346452175430137483132154566023748284704589458094199981810537483023545986277408132789499").unwrap();
        z.iter(|| &a + &b);
    }
}