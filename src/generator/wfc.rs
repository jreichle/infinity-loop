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

fn get_opposite_direction(dir: Square) -> Square {
    match dir {
        Up => Down,
        Right => Left,
        Down => Up,
        Left => Right
    }
}

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

fn parse_rules(
    available_tiles: &Vec<Tile<Square>>,
) -> HashMap<Tile<Square>, HashMap<Square, Vec<Tile<Square>>>> {

    /// Get rules set for each direction 
    /// # Arguments
    /// `is_connected` - if the rule set should be for connecting matches 
    fn parse_direction_rules(
        available_tiles: &Vec<Tile<Square>>,
        is_connected: bool,
    ) -> HashMap<Square, Vec<Tile<Square>>> {
        let mut rule_map = HashMap::new();

        // check if tile matches 
        let tile_matches: fn(&Tile<Square>, Square) -> bool = match is_connected {
            true => |tile, dir| tile.0.contains(get_opposite_direction(dir)),
            false => |tile, dir| !tile.0.contains(get_opposite_direction(dir)),
        };

        let up_tiles: Vec<Tile<Square>> = available_tiles
            .iter()
            .filter(|tile| tile_matches(tile, Up))
            .cloned()
            .collect();
        let right_tiles: Vec<Tile<Square>> = available_tiles
            .iter()
            .filter(|tile| tile_matches(tile, Right))
            .cloned()
            .collect();
        let down_tiles: Vec<Tile<Square>> = available_tiles
            .iter()
            .filter(|tile| tile_matches(tile, Down))
            .cloned()
            .collect();
        let left_tiles: Vec<Tile<Square>> = available_tiles
            .iter()
            .filter(|tile| tile_matches(tile, Left))
            .cloned()
            .collect();

        rule_map.insert(Up, up_tiles);
        rule_map.insert(Right, right_tiles);
        rule_map.insert(Down, down_tiles);
        rule_map.insert(Left, left_tiles);

        rule_map
    }

    let dir_rule_map = parse_direction_rules(available_tiles, true);
    let not_dir_rule_map = parse_direction_rules(available_tiles, false);

    let mut rule_map = HashMap::new(); //parse_rules(available_tiles);

    for tile in available_tiles.iter() {
        let mut tile_rule: HashMap<Square, Vec<Tile<Square>>> = HashMap::new();

        for dir in [Left, Down, Right, Up] {
            let dir_set;
            if tile.0.contains(dir) {
                dir_set = dir_rule_map.get(&dir).unwrap().clone();
            } else {
                dir_set = not_dir_rule_map.get(&dir).unwrap().clone();
            }
            tile_rule.insert(dir, dir_set);
        }
        rule_map.insert(tile.clone(), tile_rule);
    }
    rule_map
}

// Print all possible tiles
fn print_all_tiles() {
    let tiles = get_all_possible_tiles();
    tiles.iter().for_each(|tile| println!("{}", tile));
}