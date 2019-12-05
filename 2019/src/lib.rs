#![allow(unused_imports)]
#![allow(non_camel_case_types)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;
// #[macro_use]
// extern crate derive_new;

pub mod structures;
pub mod generators;
pub mod solutions;
pub mod test;

aoc_lib!{ year = 2019 }