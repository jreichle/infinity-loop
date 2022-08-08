use std::{collections::HashMap, fmt::Display, hash::Hash};

use enumset::EnumSet;
use rand::Rng;

use crate::model::{
    bitset::BitSet,
    coordinate::Coordinate,
    finite::Finite,
    grid::Grid,
    solver::{step, Sentinel, Superposition},
    tile::{Square, Tile},
};

impl<A: Finite + Eq + Hash + Clone + Copy + Display> BitSet<A> {
    // impl<A: Square> BitSet<A> {
    fn is_collapsed(&self) -> bool {
        self.len() <= 1
    }

    fn collapse(&mut self, weights: &HashMap<A, usize>) {
        let mut weight: f64;
        let mut option_weights: HashMap<A, f64> = HashMap::new();
        let mut total_weight: f64 = 0.0;

        let mut rng = rand::thread_rng();
        
        for cell_option in self.iter() {
            weight = weights.get(&cell_option).unwrap().clone() as f64;
            total_weight += weight;
            option_weights.insert(cell_option, weight);
        }

        let mut rng_weights = total_weight * rng.gen_range(2..10) as f64 * 0.1;
        
        for (option, weight) in option_weights.iter() {
            rng_weights -= weight;
            if rng_weights < 0.0 {
                let full_set: BitSet<A> = BitSet::FULL;
                for tile in full_set.iter() {
                    if &tile != option && self.contains(tile) {
                        self.remove(tile.clone());
                    }
                }
                break;
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct WfcGenerator {
    width: usize,
    height: usize,
    prop_limit: usize,
    pass_limit: usize,
}

impl WfcGenerator {
    fn new(width: usize, height: usize, pass_limit: usize, prop_limit: usize) -> WfcGenerator {
        WfcGenerator {
            width,
            height,
            prop_limit,
            pass_limit,
        }
    }

    fn update_weights(board: &Sentinel<Square>, weights: &mut HashMap<Tile<Square>, usize>) {
        weights.clear();

        // initialize weights
        let full_set: Superposition<Square> = BitSet::FULL;
        full_set.iter().for_each(|tile| {
            weights.insert(tile, 0);
        });

        // update weights
        for cell in board.0.elements().into_iter() {
            cell.into_iter().for_each(|tile| {
                *weights.get_mut(&tile).unwrap() += 1;
            });
        }

        // remove weights from empty edges

        // let width = board.0.columns();
        // let height = board.0.rows();
        // *weights.get_mut(&Tile(EnumSet::empty())).unwrap() -= 2 * (width + height) + 4;
    }

    fn shannon_entropy(cell: &Superposition<Square>, weights: &HashMap<Tile<Square>, usize>) -> f64 {
        let (mut weight, mut total_weight, mut total_log_weight): (f64, f64, f64);
        total_weight = 0.0;
        total_log_weight = 0.0;

        for tile in cell.iter() {
            weight = weights.get(&tile).unwrap().clone() as f64;
            total_weight += weight;
            total_log_weight += weight * weight.ln();
        }

        total_weight.ln() - (total_log_weight / total_weight)
    }

    fn find_entropy_cell(board: &Sentinel<Square>, weights: &HashMap<Tile<Square>, usize>) -> Coordinate<isize> {
        let mut min = std::f64::MAX;
        let mut min_coordinate: Coordinate<isize> = Coordinate { row: 0, column: 0 };

        let mut entropy: f64;
        let mut entropy_rng: f64;

        let mut rng = rand::thread_rng();

        for (cell_coordinate, cell) in board.0.with_index().elements().iter() {
            if cell.is_collapsed() {
                continue;
            }

            entropy = WfcGenerator::shannon_entropy(cell, weights);

            entropy_rng = entropy + rng.gen_range(1..10) as f64 * 0.000001; // add random effect -> so same value has slight different probs
                                                                            // println!("entropy: {}, entropy_rng: {}", entropy, entropy_rng);

            if entropy_rng < min {
                min = entropy_rng;
                min_coordinate = cell_coordinate.clone();
            }
        }
        min_coordinate
    }

    fn collapse_cell(board: &mut Sentinel<Square>, weights: &HashMap<Tile<Square>, usize>, cell_coordinate: Coordinate<isize>) {
        let cell = board.0.get_mut(cell_coordinate).unwrap();
        cell.collapse(weights);
    }

    fn propagate(board: &mut Sentinel<Square>, cell_coordinate: Coordinate<isize>, prop_limit: usize) {
        let mut stack: Vec<Coordinate<isize>> = vec![cell_coordinate];

        let mut passes = 0_usize;
        let mut cell: Superposition<Square>;
        while let Some(index) = stack.pop() {
            
            // TODO: cloned cell, original board unchanged.
            cell = board.0.get(index).unwrap().clone();

            if cell.is_collapsed() {
                continue;
            }

            let neighbors = index.all_neighbor_indices();

            for neighbor_index in neighbors.iter() {
                let neighbor_cell = board.0.get(neighbor_index.clone()).unwrap();
                if neighbor_cell.is_collapsed() {
                    continue;
                }

                let mut modified = false;
                let mut compatible_counter: usize = 0;
                for neighbor_tile in neighbor_cell.iter() {
                    for tile in cell.iter() {
                        // TODO: Check connections
                        _ = tile.clone();
                        let rule = vec![0];
                        if rule.contains(&0) {
                            compatible_counter += 1;
                            break;
                        }
                    }

                    if compatible_counter == 0 {
                        let neighbor_cell_mut = board.0.get_mut(neighbor_index.clone()).unwrap();
                        neighbor_cell_mut.remove(neighbor_tile);
                        modified = true;
                    }
                }

                if modified {
                    stack.push(neighbor_index.clone());
                }
            }

            passes += 1;
            if passes >= prop_limit {
                break;
            }
        }
    }

    fn is_all_collapsed(board: &Sentinel<Square>) -> bool {
        for cell in board.0.elements() {
            if !cell.is_collapsed() {
                return false;
            }
        }
        true
    }

    fn print_map(board: &Sentinel<Square>) {
        let map = board.0.elements();
        let width = board.0.columns();

        for (index, cell) in map.iter().enumerate() {
            let cell_len = cell.len();
            if cell_len > 1 {
                print!("?");
            } else if cell_len == 1 {
                print!("{}", cell.unwrap_if_singleton().unwrap());
            } else if cell_len == 0 {
                print!("!");
            }

            if index % width == width - 1 {
                print!("\n");
            }
        }
    }

    pub fn generate(&self) -> Sentinel<Square> {
        let dimension = Coordinate {
            row: self.height,
            column: self.width,
        };

        // initialize board with all possiblities, then update edge tiles
        let board: Sentinel<Square> = Grid::init(dimension, |_| BitSet::FULL)
            .with_sentinels(BitSet::singleton(Tile(EnumSet::empty())));
        let mut board = board.0.coordinates().into_iter().fold(board.clone(), step);

        let mut weights: HashMap<Tile<Square>, usize> = HashMap::new();

        // update weights
        WfcGenerator::update_weights(&board, &mut weights);

        let mut passes: usize = 0;
        let mut current_coordinate: Coordinate<isize>;

        loop {
            current_coordinate = WfcGenerator::find_entropy_cell(&board, &weights);
            WfcGenerator::collapse_cell(&mut board, &weights, current_coordinate);

            // WfcGenerator::propagate(&board,current_coordinate, self.prop_limit);
            board = board.0
                    .coordinates().into_iter()
                    .fold(board.clone(), step);

            // println!("before update {:?}", weights);
            WfcGenerator::update_weights(&board, &mut weights);
            // println!("after update {:?}", weights);

            passes += 1;

            println!("PASS #{}\n", passes);
            WfcGenerator::print_map(&board);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            if WfcGenerator::is_all_collapsed(&board) || passes >= self.pass_limit {
                break;
            }
        }

        board
    }
}

fn is_empty(board: &Sentinel<Square>) -> bool {
    for cell in board.0.elements().iter() {
        if cell.len() == 1 && cell.unwrap_if_singleton().unwrap() != Tile(EnumSet::empty()){
            return false
        }
    }
    true
}

pub fn test() {
    let wfc_generator = WfcGenerator::new(10, 10, 40000, 1000);
    let mut board = wfc_generator.generate();

    while is_empty(&board) {
        board = wfc_generator.generate();
    }

    println!("Final board: \n");
    WfcGenerator::print_map(&board);
}
