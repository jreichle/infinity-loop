#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod model;

use model::tile::*;

fn main() {
    println!("Hello, world!");

    let _tile = Tile::new(1);
}
