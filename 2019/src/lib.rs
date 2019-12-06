#![allow(unused_imports)]
#![allow(non_camel_case_types)]

extern crate aoc_runner;
extern crate itertools;

#[macro_use]
extern crate aoc_runner_derive;
// #[macro_use]
// extern crate derive_new;

pub mod structures;
pub mod generators;
pub mod solutions;
pub mod test;
pub mod day6;

aoc_lib!{ year = 2019 }