#![feature(slice_patterns)]
#[cfg_attr(thicc_ints, feature(i128_type))]

#[macro_use]
extern crate nom;

pub mod bigint;
mod rchunks;