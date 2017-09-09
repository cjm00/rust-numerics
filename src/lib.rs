#![feature(slice_patterns)]
#![cfg_attr(feature = "bench", feature(test))]
#![cfg_attr(feature = "thicc_ints", feature(i128_type))]

#[macro_use]
extern crate nom;
extern crate rchunks;

pub mod bigint;