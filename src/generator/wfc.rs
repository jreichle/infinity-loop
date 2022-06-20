// wave function collapse (WFC)

mod model;

use enumset::EnumSet;
use model::tile::{
    Square::{self, Down, Left, Right, Up},
    Tile,
};

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

// Get Tile object with given bool for all four directions (Up, Right, Down, Left)
fn get_tile_from_bool(up: bool, right: bool, down: bool, left: bool) -> Tile<Square> {
    let mut tile_enum: EnumSet<Square> = EnumSet::new();

    if up {
        tile_enum.insert_all(EnumSet::only(Up));
    }
    if right {
        tile_enum.insert_all(EnumSet::only(Right));
    }
    if down {
        tile_enum.insert_all(EnumSet::only(Down));
    }
    if left {
        tile_enum.insert_all(EnumSet::only(Left));
    }

    Tile(tile_enum)
}

// Get all possible tiles
fn get_all_possible_tiles() -> Vec<Tile<Square>> {
    let mut tiles: Vec<Tile<Square>> = vec![];
    for i in 0..=0b1111 {
        let up = (i & 1) == 1;
        let right = (i >> 1 & 1) == 1;
        let down = (i >> 2 & 1) == 1;
        let left = (i >> 3 & 1) == 1;
        let tile = get_tile_from_bool(up, right, down, left);
        tiles.push(tile);
        // println!("U: {:5}, R: {:5}, D: {:5}, L: {:5} -> {:?}", up, right, down, left, tile);
    }
    tiles
}

// Get rule set (for each direction) by given available tiles
fn parse_rules(
    available_tiles: Vec<Tile<Square>>,
) -> HashMap<Tile<Square>, HashMap<Square, Vec<Tile<Square>>>> {
    let mut rule_map = HashMap::new();
    let unique_set: HashSet<Tile<Square>> = HashSet::from_iter(available_tiles);
    let unique_tiles: Vec<Tile<Square>> = unique_set.into_iter().collect(); // remove duplicates

    for key_tile in unique_tiles.iter() {
        rule_map.insert(key_tile.clone(), HashMap::new());
        let tile_rule = rule_map.get_mut(key_tile).unwrap();

        if key_tile.0.contains(Left) {
            let left_tiles: Vec<Tile<Square>> = unique_tiles
                .iter()
                .filter(|tile| tile.0.contains(Right))
                .cloned()
                .collect();
            tile_rule.insert(Left, left_tiles); // left
        }

        if key_tile.0.contains(Down) {
            let down_tiles: Vec<Tile<Square>> = unique_tiles
                .iter()
                .filter(|tile| tile.0.contains(Up))
                .cloned()
                .collect();
            tile_rule.insert(Down, down_tiles); // down
        }

        if key_tile.0.contains(Right) {
            let right_tiles: Vec<Tile<Square>> = unique_tiles
                .iter()
                .filter(|tile| tile.0.contains(Left))
                .cloned()
                .collect();
            tile_rule.insert(Right, right_tiles); // right
        }

        if key_tile.0.contains(Up) {
            let up_tiles: Vec<Tile<Square>> = unique_tiles
                .iter()
                    .filter(|tile| tile.0.contains(Down))
                .cloned()
                .collect();
            tile_rule.insert(Up, up_tiles); // up
        }
    }
    rule_map
}

// Print all possible tiles
fn print_all_tiles() {
    let tiles = get_all_possible_tiles();
    tiles.iter().for_each(|tile| println!("{}", tile));
}