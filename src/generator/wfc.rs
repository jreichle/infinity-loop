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


// Get rules set for each direction (connection available)
fn parse_direction_rules(
    available_tiles: &Vec<Tile<Square>>,
) -> HashMap<Square, Vec<Tile<Square>>> {
    let mut rule_map = HashMap::new();

    let up_tiles: Vec<Tile<Square>> = available_tiles.iter().filter(|tile| tile.0.contains(Down)).cloned().collect();
    let right_tiles: Vec<Tile<Square>> = available_tiles.iter().filter(|tile| tile.0.contains(Left)).cloned().collect();
    let down_tiles: Vec<Tile<Square>> = available_tiles.iter().filter(|tile| tile.0.contains(Up)).cloned().collect();
    let left_tiles: Vec<Tile<Square>> = available_tiles.iter().filter(|tile| tile.0.contains(Right)).cloned().collect();

    rule_map.insert(Up, up_tiles);
    rule_map.insert(Right, right_tiles);
    rule_map.insert(Down, down_tiles);
    rule_map.insert(Left, left_tiles);

    rule_map
}

// Get rules set for without each direction (connection not available)
fn parse_not_direction_rules(
    available_tiles: &Vec<Tile<Square>>,
) -> HashMap<Square, Vec<Tile<Square>>> {
    let mut rule_map = HashMap::new();

    let up_tiles: Vec<Tile<Square>> = available_tiles.iter().filter(|tile| !tile.0.contains(Down)).cloned().collect();
    let right_tiles: Vec<Tile<Square>> = available_tiles.iter().filter(|tile| !tile.0.contains(Left)).cloned().collect();
    let down_tiles: Vec<Tile<Square>> = available_tiles.iter().filter(|tile| !tile.0.contains(Up)).cloned().collect();
    let left_tiles: Vec<Tile<Square>> = available_tiles.iter().filter(|tile| !tile.0.contains(Right)).cloned().collect();

    rule_map.insert(Up, up_tiles);
    rule_map.insert(Right, right_tiles);
    rule_map.insert(Down, down_tiles);
    rule_map.insert(Left, left_tiles);

    rule_map
}

fn parse_rules(
    available_tiles: &Vec<Tile<Square>>,
) -> HashMap<Tile<Square>, HashMap<Square, Vec<Tile<Square>>>> {
    
    let dir_rule_map = parse_direction_rules(available_tiles);
    let not_dir_rule_map = parse_not_direction_rules(available_tiles);

    let mut rule_map = HashMap::new(); //parse_rules(available_tiles);

    for tile in available_tiles.iter() {
        let mut tile_rule: HashMap<Square, Vec<Tile<Square>>> = HashMap::new();
        let (left_tiles, down_tiles, right_tiles, up_tiles): (Vec<Tile<Square>>, Vec<Tile<Square>>, Vec<Tile<Square>>, Vec<Tile<Square>>);

        if tile.0.contains(Left) {  
            left_tiles = dir_rule_map.get(&Left).unwrap().clone();
        } else {
            left_tiles = not_dir_rule_map.get(&Left).unwrap().clone();
        }
        tile_rule.insert(Left, left_tiles);
        
        if tile.0.contains(Down) {
            down_tiles = dir_rule_map.get(&Down).unwrap().clone();
        } else {
            down_tiles = not_dir_rule_map.get(&Down).unwrap().clone();
        }
        tile_rule.insert(Down, down_tiles);

        if tile.0.contains(Right) {
            right_tiles = dir_rule_map.get(&Right).unwrap().clone();
        } else {
            right_tiles = not_dir_rule_map.get(&Right).unwrap().clone();
        }
        tile_rule.insert(Right, right_tiles);


        if tile.0.contains(Up) {
            up_tiles = dir_rule_map.get(&Up).unwrap().clone();
        } else {
            up_tiles = not_dir_rule_map.get(&Up).unwrap().clone();
        }

        tile_rule.insert(Up, up_tiles);

        rule_map.insert(tile.clone(), tile_rule);
    }
    rule_map
}

// Print all possible tiles
fn print_all_tiles() {
    let tiles = get_all_possible_tiles();
    tiles.iter().for_each(|tile| println!("{}", tile));
}