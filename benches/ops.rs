#![feature(test)]

extern crate numerics;
extern crate test;

use numerics::bigint::BigInt;

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;


#[bench]
fn addition_1000_decimal_digits(b: &mut test::Bencher) {
    let mut file = File::open("./benches/inputs/1000.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut l = contents.lines();
    let x = BigInt::from_str(l.next().unwrap()).unwrap();
    let y = BigInt::from_str(l.next().unwrap()).unwrap();
    b.iter(|| &x + &y)
}

#[bench]
fn subtraction_1000_decimal_digits(b: &mut test::Bencher) {
    let mut file = File::open("./benches/inputs/1000.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut l = contents.lines();
    let x = BigInt::from_str(l.next().unwrap()).unwrap();
    let y = BigInt::from_str(l.next().unwrap()).unwrap();
    b.iter(|| &x - &y)
}

#[bench]
fn product_1000_decimal_digits(b: &mut test::Bencher) {
    let mut file = File::open("./benches/inputs/1000.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut l = contents.lines();
    let x = BigInt::from_str(l.next().unwrap()).unwrap();
    let y = BigInt::from_str(l.next().unwrap()).unwrap();
    b.iter(|| &x * &y)
}


