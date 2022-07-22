// wave function collapse (WFC)

use crate::model;
use enumset::EnumSet;
use model::grid::Grid;

use model::tile::{
    Square::{self, Down, Left, Right, Up},
    Tile,
};
use rand::Rng;
use std::collections::HashMap;

fn is_edge_index(index: usize, width: usize, height: usize) -> bool {
    if index / width == 0 || index / width == height-1 {
        true
    } else if index % width == 0 || index % width == width-1 {
        true
    } else {
        false
    }
}

fn get_opposite_direction(dir: Square) -> Square {
    match dir {
        Up => Down,
        Right => Left,
        Down => Up,
        Left => Right,
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

        // check if tile matches with or without connection
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

    // get rule map for matching tile with/without connections
    let dir_rule_map = parse_direction_rules(available_tiles, true);
    let not_dir_rule_map = parse_direction_rules(available_tiles, false);

    let mut rule_map = HashMap::new();

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

fn is_all_collapsed(map: & Vec<Vec<Tile<Square>>>) -> bool {
    for tile in map.iter() {
        if tile.len() > 1 {
            return false
        }
    }
    true
}

// find least prob cell
fn find_entropy_cell(
    cells: &Vec<Vec<Tile<Square>>>,
    tile_weights: &mut HashMap<&Tile<Square>, usize>,
) -> usize {
    let mut min = std::f64::MAX;
    let mut min_index: usize = 0;

    let mut entropy: f64;
    let mut entropy_rng: f64;

    let mut rng = rand::thread_rng();

    for (index, cell_options) in cells.iter().enumerate() {
        if cell_options.len() == 1 {
            continue;
        } // already collasped

        entropy = shannon_entropy(cell_options, tile_weights);
        entropy_rng = entropy + rng.gen_range(1..10) as f64 * 0.000001; // add random effect -> so same value has slight different probs
                                                                        // println!("entropy: {}, entropy_rng: {}", entropy, entropy_rng);

        if entropy_rng < min {
            min = entropy_rng;
            min_index = index;
        }
    }

    min_index
}

// calc prob
fn shannon_entropy(
    cell_options: &Vec<Tile<Square>>,
    tile_weights: &mut HashMap<&Tile<Square>, usize>,
) -> f64 {
    let (mut weight, mut total_weight, mut total_log_weight): (f64, f64, f64);
    total_weight = 0.0;
    total_log_weight = 0.0;

    for cell in cell_options.iter() {
        weight = tile_weights.get(cell).unwrap().clone() as f64;
        total_weight += weight;
        total_log_weight += weight * weight.ln();
    }

    total_weight.ln() - (total_log_weight / total_weight)
}

fn collapse_cell(
    cell_map: &mut Vec<Vec<Tile<Square>>>,
    cell_index: usize,
    tile_weights: &HashMap<&Tile<Square>, usize>,
) {
    let cell_options = cell_map[cell_index].clone();

    let mut weight: f64;
    let mut option_weights: Vec<f64> = Vec::new();
    let mut total_weight: f64 = 0.0;

    let mut rng = rand::thread_rng();

    for option in cell_options.iter() {
        weight = tile_weights.get(option).unwrap().clone() as f64;
        total_weight += weight;
        option_weights.push(weight);
    }

    let mut rng_weights = total_weight * rng.gen_range(2..10) as f64 * 0.1;

    for (i, weight) in option_weights.iter().enumerate() {
        rng_weights -= weight;
        if rng_weights < 0.0 {
            let new_value = vec![cell_options[i].clone()];
            cell_map[cell_index] = new_value;
            break;
        }
    }
}


fn propagate(
    cell_map: &mut Vec<Vec<Tile<Square>>>,
    map_size: (usize, usize),
    cell_index: usize,
    tile_weights: &mut HashMap<&Tile<Square>, usize>,
    rule_map: &HashMap<Tile<Square>, HashMap<Square, Vec<Tile<Square>>>>,
) {

    let (width, height) = map_size;
    let mut stack: Vec<usize> = vec![cell_index];

    fn get_row_col_index(index: usize, width: usize, height: usize) -> (isize, isize) {
        let row = (index / width) as isize;
        let col = (index % width) as isize;
        (row, col)
    }

    fn get_index_by_row_col(row: isize, col: isize, width: usize) -> usize {
        row as usize * width + col as usize
    }

    fn get_index_by_dir(cell_index: usize, width: usize, height: usize, dir: Square) -> (isize, isize) {
        let (mut row, mut col) = get_row_col_index(cell_index, width, height);

        match dir {
            Up => row -= 1,
            Right => col += 1,
            Down => row += 1,
            Left => col -= 1,
        }

        if col < 0 || col >= width as isize || row < 0 || row >= height as isize {
            row = -1;
            col = -1;
        }

        (row, col)
    }

    let mut passes = 0_usize;
    let mut compactible_tiles: Vec<Tile<Square>>;

    while let Some(index) = stack.pop() {
        compactible_tiles = cell_map[index].clone();

        if compactible_tiles.len() == 0 {
            // println!("cell {} empty - skip!", index);
            continue;
        }

        for dir in [Up, Right, Down, Left] {
            let (row, col) = get_index_by_dir(index, width, height, dir);
            if row == -1 || col == -1 {
                continue;
            } // skip if out of bound

            if is_edge_index(index, width, height) {
                continue;
            }

            let neigbor_index = get_index_by_row_col(row, col, width);
            let neigbor_tiles = &mut cell_map[neigbor_index];

            // prevent collaspe neigbors getting propagated
            if neigbor_tiles.len() == 1 {
                continue;
            }

            let mut modified = false;

            let mut remove_list: Vec<usize> = vec![];
            for (i, neigbor_tile) in neigbor_tiles.iter().enumerate() {

                // let neigbor_tile = neigbor_tiles[i];
                let mut compatible_counter = 0;
                for tile in compactible_tiles.iter() {
                    let tile_dir_rule = rule_map.get(tile).unwrap().get(&dir).unwrap();
                    if tile_dir_rule.contains(&neigbor_tile) {
                        compatible_counter += 1;
                        break;
                    }
                }

                if compatible_counter == 0 {
                    // remove function shifts vector
                    // neigbor_tiles.remove(i);
                    remove_list.push(i);
                    modified = true;
                }
            }

            let mut remove_counter = 0_usize; 
            neigbor_tiles.retain(|tile| { remove_counter += 1; !remove_list.contains(&(remove_counter - 1)) });

            if modified {
                stack.push(neigbor_index)
            }
        }

        passes += 1;
        if passes >= PROP_LMT {
            break;
        }
    }
}

fn print_incomplete_map(map: & Vec<Vec<Tile<Square>>>, width: usize) {
    for (index, cell) in map.iter().enumerate() {
        let cell_len = cell.len();
        if cell_len > 1 {
            print!(" ");
        } else if cell_len == 1 {
            print!("{}", cell[0]);
        } else if cell_len == 0 {
            print!("?");
        }

        if index % width == width-1 {
            print!("\n");
        }
        // println!("cell -> {:?}\n\n", cell);
    }    
}

// 1. Generate tiles and rules
// 2. Create matrix with each cell filled with all possiblie tiles
// 3. Find min Entropy (cell with least weight, least likely)
// 4. Pick random cell to continue -> collapse_cell
// 5. Propagate, check picked cell neighbors.
// DO...WHILE(!is_all_collapsed())


static PASS_LMT: usize = 400; // How many passes to go through the matrix
static PROP_LMT: usize = 1000; // How many passes in the propagate to allow
static WIDTH: usize = 5;
static HEIGHT: usize = 5;

fn generate_grid(
    width: usize,
    height: usize,
    available_tiles: Vec<Tile<Square>>,
) -> Grid<Tile<Square>> {
    let rule_map = parse_rules(&available_tiles);
    
    let total_cells = width * height;

    let wrapper_width = width + 2;
    let wrapper_height = height + 2;
    let wrapper_size = wrapper_width * wrapper_height;

    // let mut cell_map: Vec<Vec<Tile<Square>>> = vec![available_tiles.clone(); wrapper_size];
    let mut cell_map: Vec<Vec<Tile<Square>>> = vec![available_tiles.clone(); wrapper_size];

    for index in 0..wrapper_size {
        
        // check if top or buttom edge
        if is_edge_index(index, wrapper_width, wrapper_height) {
            cell_map[index] = vec![Tile(EnumSet::empty())];
            continue;
        }

        let up_index = index - wrapper_width;
        let bt_index = index + wrapper_width;
        let lf_index = index - 1;
        let rt_index = index + 1;

        // print!("index={} -> ", index);
        // print!("\nbefore -> {:?}\n", cell_map[index]);
        if is_edge_index(up_index, wrapper_width, wrapper_height) {
            // print!("up index={}, ", up_index);
            cell_map[index].retain(|tile| !tile.0.contains(Up));
        }
        if is_edge_index(bt_index, wrapper_width, wrapper_height) {
            // print!("bt index={}, ", bt_index);
            cell_map[index].retain(|tile| !tile.0.contains(Down));
        }
        if is_edge_index(lf_index, wrapper_width, wrapper_height) {
            // print!("lf index={}, ", lf_index);
            cell_map[index].retain(|tile| !tile.0.contains(Left));
        }
        if is_edge_index(rt_index, wrapper_width, wrapper_height) {
            // print!("rt index={}", rt_index);
            cell_map[index].retain(|tile| !tile.0.contains(Right));
        }
        // print!("after -> {:?}\n", cell_map[index]);
        // print!("\n");
    }

    let mut tile_weights: HashMap<&Tile<Square>, usize> = HashMap::new();
    for tile in available_tiles.iter() {
        tile_weights.insert(tile, total_cells);
    }

    let mut current_index: usize;
    let mut passes = 0;

    loop {
        current_index = find_entropy_cell(&cell_map, &mut tile_weights);
        collapse_cell(&mut cell_map, current_index, &tile_weights);
        propagate(&mut cell_map, (wrapper_width, wrapper_height), current_index, &mut tile_weights, &rule_map);

        passes += 1;

        if is_all_collapsed(&cell_map) || passes >= PASS_LMT {
            break;
        }
    }

    // final map
    println!("FINAL MAP");
    print_incomplete_map(&cell_map, wrapper_width);

    println!("PASS COUNT: {}", passes);


    let mut counter = 0;
    for cell in cell_map.iter_mut() {
        if cell.len() != 1 {
            *cell = vec![Tile(EnumSet::empty())];
            counter += 1;
        }
    }
    println!("invaild count: {}", counter);

    let flat_map: Vec<Tile<Square>> = cell_map.into_iter().flatten().collect();
    // println!("flat map length: {}", flat_map.len());

    let game_board: Grid<Tile<Square>> = Grid::new(wrapper_width, wrapper_height, flat_map);
    game_board
}
