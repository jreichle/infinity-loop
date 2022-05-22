#![allow(dead_code)]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod model;

use model::board::*;
use model::tile::*;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();

    let rows = 10;
    let columns = 10;

    let random_vals = (0..rows * columns)
        .map(|_| Tile::new(rng.gen_range(0..16)))
        .collect();

    let mut board = Board::new(rows, columns, random_vals);
    println!("{board}");
    println!("{}", "-".repeat(rows));
    board = board.map(|t| t.rotate_clockwise(1));
    println!("{}", board)
}
