#![allow(dead_code)]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod generator;
mod model;
mod view;

use model::{
    grid::Grid,
    tile::{Square, Tile},
};
use std::{env, time::Instant};
use view::window::*;

use crate::generator::levelstream::{level_stream, LevelProperty};

#[allow(unused_variables)] // for testing purposes
fn main() {
    const SHOW_ERROR_CALLSTACK: bool = true;

    if SHOW_ERROR_CALLSTACK {
        env::set_var("RUST_BACKTRACE", "1");
    }

    // initiate_window();

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
