use std::{
    collections::HashMap,
    hash::Hash
};

use enumset::EnumSet;
use rand::Rng;

use crate::model::{
    bitset::BitSet,
    coordinate::Coordinate,
    finite::Finite,
    grid::Grid,
    solver::{Sentinel, Superposition, step},
    tile::{
        Square,
        Tile,
    },
};

#[derive(Clone, PartialEq, Eq)]
struct WfcGenerator {
    board: Sentinel<Square>,
    weights: HashMap<Tile<Square>, usize>,
    pass_limit: usize,
    prop_limit: usize,
}

impl<A: Finite + Eq + Hash + Clone + Copy> BitSet<A> { 
// impl<A: Square> BitSet<A> { 
    fn is_collapsed(&self) -> bool {
        self.len() <= 1
    }

    fn collapse(self, weights: &HashMap<A, usize>) {
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
                // TODO
                // self.remove(element);
                // self.singleton(element)
                // self::BitSet::EMPTY.insert();
            }
        }
    }
}

impl WfcGenerator {
    fn new(width: usize, height: usize) -> WfcGenerator {
        let dimension = Coordinate { row: height, column: width };
        
        // initialize board with all possiblities, then update edge tiles
        let board: Sentinel<Square> = Grid::init(dimension, |_| BitSet::FULL)
                .with_sentinels(BitSet::singleton(Tile(EnumSet::empty())));
        let board = board.0.coordinates().into_iter().fold(board.clone(), step);

        let mut weights: HashMap<Tile<Square>, usize> = HashMap::new();

        // initialize weights
        let full_set: Superposition<Square> = BitSet::FULL;
        full_set.iter().for_each(| tile | {
            weights.insert(tile, 0);
        });

        // update weights
        for cell in board.0.elements().into_iter() {
            cell.into_iter().for_each(|tile| {
                *weights.get_mut(&tile).unwrap() += 1;
            });  
        }

        // remove weights from empty edges
        *weights.get_mut(&Tile(EnumSet::empty())).unwrap() -= 2*(width+height) + 4;

        let pass_limit: usize = 4000;
        let prop_limit: usize = 1000;

        WfcGenerator { board, weights, pass_limit, prop_limit }
    }

    fn shannon_entropy(&self, cell: &Superposition<Square>) -> f64 {
        let (mut weight, mut total_weight, mut total_log_weight): (f64, f64, f64);
        total_weight = 0.0;
        total_log_weight = 0.0;

        for tile in cell.iter() {
            weight = self.weights.get(&tile).unwrap().clone() as f64;
            total_weight += weight;
            total_log_weight += weight * weight.ln();
        }
    
        total_weight.ln() - (total_log_weight / total_weight)        

    }

    fn find_entropy_cell(&self) -> Coordinate<isize> {
        let mut min = std::f64::MAX;
        let mut min_coordinate: Coordinate<isize> = Coordinate { row: 0, column: 0 };

        let mut entropy: f64;
        let mut entropy_rng: f64;

        let mut rng = rand::thread_rng();

        for (cell_coordinate, cell) in self.board.0.with_index().elements().iter() {
            if cell.is_collapsed() {
                continue;
            }

            entropy = self.shannon_entropy(&cell);
            entropy_rng = entropy + rng.gen_range(1..10) as f64 * 0.000001; // add random effect -> so same value has slight different probs
                                                                            // println!("entropy: {}, entropy_rng: {}", entropy, entropy_rng);
    
            if entropy_rng < min {
                min = entropy_rng;
                min_coordinate = cell_coordinate.clone();
            }
        }
        min_coordinate
    }

    
    fn collapse_cell(&self, cell_coordinate: Coordinate<isize>) {
        let cell = self.board.0.get(cell_coordinate).unwrap();
        cell.collapse(&self.weights);
    }

    fn propagate(&self, cell_coordinate: Coordinate<isize>) {
        let mut stack: Vec<Coordinate<isize>> = vec![cell_coordinate];

        let mut passes = 0_usize;
        let mut cell: Superposition<Square>;
        while let Some(index) = stack.pop() {
            cell = self.board.0.get(index).unwrap().clone();

            if cell.is_collapsed() {
                continue;
            }

            let neighbors = index.all_neighbor_indices();


            for neighbor_index in neighbors.iter() {
                let neighbor_cell = self.board.0.get(neighbor_index.clone()).unwrap();
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
                        if rule.contains(&0){
                            compatible_counter += 1;
                            break;
                        }
                    }

                    if compatible_counter == 0 {
                        neighbor_cell.remove(neighbor_tile);
                        modified = true;
                    }
                }

                if modified {
                    stack.push(neighbor_index.clone());
                }
            }

            passes += 1;
            if passes >= self.pass_limit {
                break;
            }


        }
    }

    fn is_all_collapsed(&self) -> bool {
        for cell in self.board.0.elements() {
            if !cell.is_collapsed() {
                return false;
            }
        }
        true
    }

    fn print_incomplete_map(map: &Vec<Superposition<Square>>, width: usize) {
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

    fn generate(self) {

        let mut passes: usize = 0;
        let mut current_coordinate: Coordinate<isize>;

        loop {
            current_coordinate = self.find_entropy_cell();
            self.collapse_cell(current_coordinate);
            self.propagate(current_coordinate);
            passes += 1;

            println!("PASS #{}\n", passes);
            WfcGenerator::print_incomplete_map(&self.board.0.elements(), 12);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            if self.is_all_collapsed() || passes >= self.pass_limit {
                break;
            }
        }

        println!("Final board: {:?}", self.board);

    }

}

pub fn test() {
    let wfc_grid = WfcGenerator::new(10, 10);
    // for (tile, count) in wfc_grid.weights.into_iter() {
    //     println!("{} => {}", tile, count);
    // }

    wfc_grid.generate();
}

pub fn generate_level(
    width: usize,
    height: usize,
    available_tiles: Vec<Tile<Square>>,
) -> Grid<Tile<Square>> {
    // let dimension = Coordinate::new(height, width);
    // let sentinel = Grid::init(dimension, |_| BitSet::FULL)
    //                     .with_sentinels(BitSet::singleton(Tile(EnumSet::empty())));

    let level = Grid::new(
        Coordinate::new(width, height),
        vec![available_tiles[0].clone(); width * height],
    );

    level
}
