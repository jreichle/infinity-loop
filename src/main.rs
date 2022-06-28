#![allow(dead_code)]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod model;

use std::env;

use model::testlevel::*;

use crate::model::solver::*;

fn main() {
    const SHOW_ERROR_CALLSTACK: bool = false;

    if SHOW_ERROR_CALLSTACK {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let _levels = TEST_LEVELS
        .map(|l| parse_level(l, char_to_tile).unwrap())
        .to_vec();
    // levels
    //     .iter()
    //     .zip(1..)
    //     .for_each(|(l, i)| println!("level {i}\n{l}\n"));

    let level = parse_level(TRIVIAL_LEVEL, char_to_tile).unwrap();

    println!("{}", level);
    println!("{}", solve(&level));
}
