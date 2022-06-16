#![allow(dead_code)]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod model;

use model::testlevel::*;

fn main() {
    let levels = TEST_LEVELS
        .map(|l| parse_level(l, char_to_tile).unwrap())
        .to_vec();
    levels
        .iter()
        .zip(1..)
        .for_each(|(l, i)| println!("level {i}\n{l}\n"));
}
