use rand::Rng;
use std::{fmt::Display, hash::Hash};

use crate::model::{
    coordinate::Coordinate,
    enumset::EnumSet,
    finite::Finite,
    grid::Grid,
    solver::{propagate_restrictions_to_all_neighbors, Sentinel, Superposition},
    tile::{
        Square::{self},
        Tile,
    }, enummap::EnumMap,
};

const PRINT_INTERMEDIATE_RESULTS: bool = false;

impl<A: Finite + Eq + Hash + Clone + Copy + Display> EnumSet<A> {
    fn is_collapsed(&self) -> bool {
        self.len() <= 1
    }

    fn collapse(&mut self, weights: &EnumMap<A, usize>) {
        let mut weight: f64;
        let mut option_weights: EnumMap<A, f64> = EnumMap::empty();
        let mut total_weight: f64 = 0.0;

        let mut rng = rand::thread_rng();

        for cell_option in self.iter() {
            weight = weights[cell_option].unwrap() as f64;
            total_weight += weight;
            option_weights.insert(cell_option, weight);
        }

        let mut rng_weights = total_weight * rng.gen_range(2..10) as f64 * 0.1;

        for (option, weight) in option_weights.iter() {
            rng_weights -= weight;
            if rng_weights < 0.0 {
                for tile in A::all_enums_ascending() {
                    if tile != option {
                        self.remove(tile);
                    }
                }
                break;
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct WfcGenerator {
    width: usize,
    height: usize,
    available_tiles: EnumSet<Tile<Square>>,
    prop_limit: usize,
    pass_limit: usize,
}

impl WfcGenerator {
    pub fn new(
        width: usize,
        height: usize,
        available_tiles: EnumSet<Tile<Square>>,
        pass_limit: usize,
        prop_limit: usize,
    ) -> WfcGenerator {
        WfcGenerator {
            width,
            height,
            available_tiles,
            prop_limit,
            pass_limit,
        }
    }

    fn update_weights(board: &Sentinel<Square>, weights: &mut EnumMap<Tile<Square>, usize>) {
        weights.clear();

        // initialize weights
        let full_set: Superposition<Square> = EnumSet::FULL;
        full_set.iter().for_each(|tile| {
            weights.insert(tile, 0);
        });

        // update weights: only calculate weight for uncollapsed cells
        for cell in board.0.elements().into_iter() {
            if !cell.is_collapsed() {
                cell.into_iter().for_each(|tile| {
                    weights[tile] = weights[tile].map(|x| x + 1);
                });
            }
        }
    }

    fn shannon_entropy(
        cell: &Superposition<Square>,
        weights: &EnumMap<Tile<Square>, usize>,
    ) -> f64 {
        let (mut weight, mut total_weight, mut total_log_weight): (f64, f64, f64);
        total_weight = 0.0;
        total_log_weight = 0.0;

        for tile in cell.iter() {
            weight = weights[tile].unwrap() as f64;
            total_weight += weight;
            total_log_weight += weight * weight.ln();
        }

        total_weight.ln() - (total_log_weight / total_weight)
    }

    fn find_entropy_cell(
        board: &Sentinel<Square>,
        weights: &EnumMap<Tile<Square>, usize>,
    ) -> Coordinate<isize> {
        let mut min = std::f64::MAX;
        let mut min_coordinate: Coordinate<isize> = Coordinate { row: 0, column: 0 };

        let mut entropy: f64;
        let mut entropy_rng: f64;

        let mut rng = rand::thread_rng();

        for (cell_coordinate, cell) in board
            .0
            .with_index()
            .elements()
            .iter()
            .filter(|(_, c)| !c.is_collapsed())
        {
            entropy = WfcGenerator::shannon_entropy(cell, weights);
            // add random effect -> so same value has slight different probs
            entropy_rng = entropy + rng.gen_range(1..10) as f64 * 0.000001;

            if entropy_rng < min {
                min = entropy_rng;
                min_coordinate = *cell_coordinate;
            }
        }
        min_coordinate
    }

    fn collapse_cell(
        board: &mut Sentinel<Square>,
        weights: &EnumMap<Tile<Square>, usize>,
        cell_coordinate: Coordinate<isize>,
    ) {
        board.0.get_mut(cell_coordinate).unwrap().collapse(weights)
    }

    fn propagate(
        board: &mut Sentinel<Square>,
        cell_coordinate: Coordinate<isize>,
        prop_limit: usize,
    ) {
        fn is_compatible(
            source_tile: Tile<Square>,
            dir: Square,
            target_tile: Tile<Square>,
        ) -> bool {
            source_tile.0.contains(dir) == target_tile.0.contains(-dir)
        }

        let mut stack: Vec<Coordinate<isize>> = vec![cell_coordinate];

        let mut passes = 0_usize;
        while let Some(index) = stack.pop() {
            for dir in Square::all_enums_ascending() {
                let neighbor_index = index.get_neighbor_index(dir);
                let neighbor_cell = &board.0.get(neighbor_index).unwrap();

                if neighbor_cell.is_collapsed() {
                    continue;
                }

                let mut modified = false;
                for neighbor_tile in neighbor_cell.iter() {
                    let mut compatible_counter: usize = 0;
                    for tile in board.0.get(index).unwrap().iter() {
                        if is_compatible(tile, dir, neighbor_tile) {
                            compatible_counter += 1;
                            break;
                        }
                    }

                    if compatible_counter == 0 {
                        let neighbor_cell = board.0.get_mut(neighbor_index).unwrap();
                        neighbor_cell.remove(neighbor_tile);
                        modified = true;
                    }
                }

                if modified {
                    stack.push(neighbor_index);
                }
            }

            passes += 1;
            if passes >= prop_limit {
                break;
            }
        }
    }

    fn is_all_collapsed(board: &Sentinel<Square>) -> bool {
        board.0.as_slice().iter().all(|c| c.is_collapsed())
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
                println!();
            }
        }
    }

    pub fn generate(&self) -> Result<Grid<Tile<Square>>, String> {
        let dimension = Coordinate::new(self.height, self.width);

        // initialize board with all possiblities, then update edge tiles
        let board: Sentinel<Square> = Grid::init(dimension, |_| self.available_tiles)
            .with_sentinels(Tile::NO_CONNECTIONS.into())
            .minimize();

        // initialize superpositions
        let mut board = board
            .0
            .coordinates()
            .into_iter()
            .fold(board, propagate_restrictions_to_all_neighbors);

        let mut weights: EnumMap<Tile<Square>, usize> = EnumMap::empty();
        // update weights
        WfcGenerator::update_weights(&board, &mut weights);

        let mut passes: usize = 0;
        let mut current_coordinate: Coordinate<isize>;

        loop {
            current_coordinate = WfcGenerator::find_entropy_cell(&board, &weights);
            WfcGenerator::collapse_cell(&mut board, &weights, current_coordinate);
            WfcGenerator::propagate(&mut board, current_coordinate, self.prop_limit);
            WfcGenerator::update_weights(&board, &mut weights);

            passes += 1;

            if PRINT_INTERMEDIATE_RESULTS {
                println!("PASS #{}\n", passes);
                WfcGenerator::print_map(&board);
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            }

            if WfcGenerator::is_all_collapsed(&board) || passes >= self.pass_limit {
                break;
            }
        }
        board.extract_if_collapsed().ok_or_else(|| "ERROR".into())
    }
}

#[cfg(test)]
mod tests {

    use crate::generator::wfc::WfcGenerator;
    use crate::model::{
        enumset::EnumSet,
        tile::{
            Square::{self, Down, Left, Right, Up},
            Tile,
        },
    };
    use crate::{enumset, tile};

    #[quickcheck]
    fn wfc_test_full_set() -> bool {
        let available_tiles = EnumSet::<Tile<Square>>::FULL;
        wfc_test(16, 10, available_tiles, 40000, 1000)
    }

    #[quickcheck]
    fn wfc_test_part_set() -> bool {
        let available_tiles = enumset!(
            Tile::NO_CONNECTIONS,
            tile!(Right, Down),
            tile!(Up, Right),
            tile!(Down, Left),
            tile!(Up, Left)
        );
        wfc_test(6, 6, available_tiles, 40000, 1000)
    }

    fn wfc_test(
        width: usize,
        height: usize,
        available_tiles: EnumSet<Tile<Square>>,
        pass_limit: usize,
        prop_limit: usize,
    ) -> bool {
        let wfc_generator =
            WfcGenerator::new(width, height, available_tiles, pass_limit, prop_limit);
        let mut generation_result = wfc_generator.generate();

        while let Err(_) = generation_result {
            generation_result = wfc_generator.generate();
        }

        match generation_result {
            Ok(board) => {
                println!("Final board: ");
                println!("{}", format!("{}", board.to_string()));
                if &board.solve().count() >= &1 {
                    println!("[O] level solvable.");
                    println!("----------------------------");
                    return true;
                } else {
                    println!("[X] level not solvable.");
                    println!("----------------------------");
                    return false;
                }
            }
            Err(msg) => {
                println!("{}", msg);
                return false;
            }
        }
    }
}
