#![allow(dead_code)]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod core;
pub mod generator;
pub mod model;
pub mod solver;

use model::{
    grid::Grid,
    tile::{Square, Tile},
};
use std::{env, time::Instant};

use crate::generator::levelstream::{level_stream, LevelProperty};

#[allow(unused_variables)] // for testing purposes
fn lib() {
    const SHOW_ERROR_CALLSTACK: bool = true;

    if SHOW_ERROR_CALLSTACK {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let property = LevelProperty {
        dimension: 5.into(),
    };

    level_stream(property)
        .zip(1..)
        .take(60)
        .for_each(|(l, n)| print_level_and_solutions(&l(n), &format!("generated {n}")));
}

fn print_level_and_solutions(level: &Grid<Tile<Square>>, level_identifier: &str) {
    println!("\nlevel {level_identifier}\n{level}\n");
    level.solve().zip(1..).for_each(|(s, n)| {
        let start = Instant::now();

        println!("level {level_identifier} solution {n}\n{s}\n");
        let duration = start.elapsed().as_millis();
        println!("{duration}ms")
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
