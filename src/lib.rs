//! A general purpose numerics library

#![feature(slice_patterns)]
#![cfg_attr(feature = "bench", feature(test))]
#![cfg_attr(feature = "thicc_ints", feature(i128_type))]
#![warn(missing_docs)]

#[macro_use]
extern crate nom;
extern crate rchunks;
extern crate rand;

pub mod bigint;
