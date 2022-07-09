#![allow(dead_code)]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod model;
mod view;


use std::{env, time::Instant};
use view::window::*;
use model::{
    grid::Grid,
    testlevel::*,
    tile::{Square, Tile},
};

use crate::model::solver::*;

#[allow(unused_variables)] // for testing purposes
fn main() {
    const SHOW_ERROR_CALLSTACK: bool = true;

    if SHOW_ERROR_CALLSTACK {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let levels = TEST_LEVELS
        .map(|l| parse_level(l, char_to_tile).unwrap())
        .to_vec();

    // levels
    //     .iter()
    //     .zip(1..)
    //     .for_each(|(l, i)| print_level_and_solutions(l, &i.to_string()));

    // print_level_and_solutions(
    //     &parse_level(MULTIPLE_SOLUTIONS, char_to_tile).unwrap(),
    //     "multiple",
    // );

    let start = Instant::now();
    println!("{}", parse_level(HARD_LEVEL, char_to_tile).ok().and_then(|p| solve(&p).next()).unwrap());
    let duration = start.elapsed().as_millis();
    println!("{duration}ms")

    // initiate_window();
}

fn print_level_and_solutions(level: &Grid<Tile<Square>>, level_identifier: &str) {
    println!("\nlevel {level_identifier}\n{level}\n");
    solve(level)
        .into_iter()
        .zip(1..)
        .for_each(|(s, n)| {
            let start = Instant::now();
            
            println!("level {level_identifier} solution {n}\n{s}\n");
            let duration = start.elapsed().as_millis();
            println!("{duration}ms")
            
        })
}
