#![allow(dead_code)]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod model;
mod view;

//use model::grid::*;
//use model::squaretile::*;
//use model::testlevel::*;


use view::window::*;

fn main() {
    initiate_window();

    // let levels = TEST_LEVELS
    //     .map(|l| parse_level(l, char_to_tile).unwrap())
    //     .to_vec();
    // levels
    //     .iter()
    //     .zip(1..)
    //     .for_each(|(l, i)| println!("level {i}\n{l}\n"));
}
