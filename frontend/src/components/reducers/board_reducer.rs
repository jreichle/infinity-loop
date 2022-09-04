#![allow(dead_code)]

use rand::Rng;
use std::rc::Rc;
use yew::prelude::*;

use crate::helper::level_randomizer::randomize_level;
use crate::helper::local_storage::save_level;
use wasm_bindgen::{prelude::*, JsCast};

use game::generator::{fastgen::generate, wfc::WfcGenerator};
use game::model::{
    coordinate::Coordinate,
    gameboard::GameBoard,
    grid::Grid,
    tile::{Square, Tile},
};
use game::solver::hint::{generate_solving_trace, get_hint};

use game::core::finite::Finite;

/// reducer facilitates actions for both the board and the editor pages
///
/// playing board actions:
/// - TurnCell: turns the cell indicated through the coordinate clockwise
/// - ReplaceGrid: replaces the current grid with a new one
/// - NextLevel: generates the next level with dimension + 1 and sets it as the current level
/// - GetHint: generates a hint and highlights the corresponding tile
/// - SolveLevel: solves the level
///
/// editor board actions:
/// - ChangeTileShape: Changes the shape of given cell (iterates through all shapes)
/// - ChangeSize: Change the size of the level, and re-generates a new level.
/// - GenerateFastGen: Generates a new level with the FastGen generator.
/// - GenerateWFC: Generates a new level with the Wave functioncollapse generator.
/// - ShuffleTileRotations: Randomly rotates each cell of the level.
/// - ClearGrid: Replaces the current level with an empty grid.
pub enum BoardAction {
    // playing level actions
    TurnCell(Coordinate<isize>),
    ReplaceGrid(Grid<Tile<Square>>),
    NextLevel,
    GetHint,
    SolveLevel,
    SolveLevelInput(String),

    // Editor actions
    ChangeTileShape(Coordinate<isize>),
    ChangeSize(Coordinate<usize>),
    GenerateFastGen,
    GenerateWFC,
    ShuffleTileRotations,
    ClearGrid,
}

// reducer's state
// is a functor
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Level<A> {
    pub id: usize,
    pub data: A,
}

impl<A> Level<A> {
    fn new(id: usize, data: A) -> Self {
        Level { id, data }
    }
}

#[wasm_bindgen]
pub fn highlight_cells(row: usize, column: usize) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let cell = document
        .get_element_by_id(&format!("cell-r-{}-c-{}", row, column))
        .unwrap();

    let class_names = cell.get_attribute("class").unwrap();

    // prevent re-highlighting if already highlighted
    if class_names.contains("cell-hint-highlight") {
        return;
    }

    let highlight_class_names = format!("{} {}", class_names, "cell-hint-highlight");
    cell.set_class_name(&highlight_class_names);
    let hl = Closure::<dyn Fn()>::new(move || {
        cell.set_class_name(&class_names);
    });

    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(hl.as_ref().unchecked_ref(), 500)
        .ok();
    hl.forget();
}

impl Reducible for Level<Grid<Tile<Square>>> {
    type Action = BoardAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            BoardAction::TurnCell(index) => {
                Level::new(self.id, self.data.rotate_clockwise(index).unwrap()).into()
            }
            BoardAction::ReplaceGrid(grid) => Level::new(self.id, grid).into(),
            BoardAction::NextLevel => {
                let data = randomize_level(generate(self.data.dimensions() + 1, self.id as u64));
                save_level(&data);
                Level::new(self.id + 1, data).into()
            }
            BoardAction::GetHint => {
                let trace = generate_solving_trace(&self.data);
                log::info!("trace: {:?}", trace);
                if let Ok(coordinate) = get_hint(&self.data, trace) {
                    highlight_cells(coordinate.row as usize, coordinate.column as usize);
                    log::info!("Highlighting: {}", coordinate);
                }
                self
            }
            BoardAction::SolveLevel => match self.data.solve().next() {
                None => self,
                Some(solution) => {
                    log::info!("solved level:\n{solution}");
                    save_level(&solution);
                    Level::new(self.id, solution).into()
                }
            },
            BoardAction::SolveLevelInput(input) => {
                let solved_version = self.data.solve_with_input(&input);
                log::info!("solved level:\n {}", solved_version);

                // let mut same = true;

                // let first_grid = solved_version.as_slice().clone();
                // let second_grid = self.data.as_slice().clone();

                // for i in 0..first_grid.len() {
                //     if !check_match(first_grid[i], second_grid[i]) {
                //         same = false;
                //     }
                // }

                let same = self
                    .data
                    .clone()
                    .into_iter()
                    .zip(solved_version.clone().into_iter())
                    .all(|(x, y)| check_match(x, y));
                if solved_version.is_solved() && same {
                    Level::new(self.id, solved_version).into()
                } else {
                    self
                }
            }

            // Editor actions
            BoardAction::ChangeTileShape(index) => {
                log::info!("Change tile shape");
                Level::new(self.id, self.data.change_tile_shape(index).unwrap()).into()
            }
            BoardAction::ChangeSize(size) => Level::new(
                self.id,
                generate(size, rand::thread_rng().gen_range(0..10000)),
            )
            .into(),
            BoardAction::GenerateFastGen => {
                let data = generate(
                    self.data.dimensions(),
                    rand::thread_rng().gen_range(0..10000),
                );
                log::info!("Generated grid\n{data}");
                Level::new(self.id, data).into()
            }
            BoardAction::GenerateWFC => {
                let wfc_settings =
                    WfcGenerator::with_all_tiles(self.data.columns(), self.data.rows());
                let data = retry_until_ok(wfc_settings, WfcGenerator::generate);

                log::info!("Generated grid\n{data}");
                Level::new(self.id, data).into()
            }
            BoardAction::ShuffleTileRotations => {
                let data = randomize_level(self.data.clone());
                save_level(&data);
                log::info!("Tile rotations shuffled\n{data}");
                Level::new(self.id, data).into()
            }
            BoardAction::ClearGrid => {
                let data = Grid::filled_with(self.data.dimensions(), Tile::NO_CONNECTIONS);
                Level::new(self.id, data).into()
            }
        }
    }
}

impl Level<Grid<Tile<Square>>> {
    pub fn set_size(dimensions: Coordinate<usize>) -> impl Fn() -> Level<Grid<Tile<Square>>> {
        move || Level {
            id: 1,
            data: generate(dimensions, 1),
        }
    }

    pub fn set_grid(grid: Grid<Tile<Square>>) -> impl Fn() -> Level<Grid<Tile<Square>>> {
        move || Level {
            id: 1,
            data: grid.clone(),
        }
    }
}

//checks if 2 tiles have the same playing piece on them, no matter the rotation
pub fn check_match(first: Tile<Square>, second: Tile<Square>) -> bool {
    match first.enum_to_index() {
        0 => second.enum_to_index() == 0,
        1 => {
            second.enum_to_index() == 1
                || second.enum_to_index() == 2
                || second.enum_to_index() == 4
                || second.enum_to_index() == 8
        }
        2 => {
            second.enum_to_index() == 1
                || second.enum_to_index() == 2
                || second.enum_to_index() == 4
                || second.enum_to_index() == 8
        }
        4 => {
            second.enum_to_index() == 1
                || second.enum_to_index() == 2
                || second.enum_to_index() == 4
                || second.enum_to_index() == 8
        }
        8 => {
            second.enum_to_index() == 1
                || second.enum_to_index() == 2
                || second.enum_to_index() == 4
                || second.enum_to_index() == 8
        }

        3 => {
            second.enum_to_index() == 3
                || second.enum_to_index() == 6
                || second.enum_to_index() == 9
                || second.enum_to_index() == 12
        }
        6 => {
            second.enum_to_index() == 3
                || second.enum_to_index() == 6
                || second.enum_to_index() == 9
                || second.enum_to_index() == 12
        }
        9 => {
            second.enum_to_index() == 3
                || second.enum_to_index() == 6
                || second.enum_to_index() == 9
                || second.enum_to_index() == 12
        }
        12 => {
            second.enum_to_index() == 3
                || second.enum_to_index() == 6
                || second.enum_to_index() == 9
                || second.enum_to_index() == 12
        }

        5 => second.enum_to_index() == 5 || second.enum_to_index() == 10,
        10 => second.enum_to_index() == 5 || second.enum_to_index() == 10,

        7 => {
            second.enum_to_index() == 7
                || second.enum_to_index() == 11
                || second.enum_to_index() == 13
                || second.enum_to_index() == 14
        }
        11 => {
            second.enum_to_index() == 7
                || second.enum_to_index() == 11
                || second.enum_to_index() == 13
                || second.enum_to_index() == 14
        }
        13 => {
            second.enum_to_index() == 7
                || second.enum_to_index() == 11
                || second.enum_to_index() == 13
                || second.enum_to_index() == 14
        }
        14 => {
            second.enum_to_index() == 7
                || second.enum_to_index() == 11
                || second.enum_to_index() == 13
                || second.enum_to_index() == 14
        }

        15 => second.enum_to_index() == 15,

        _ => false,
    }
}

/// Retries an impure computation until it succeeds
fn retry_until_ok<A, B, E, F: Fn(&A) -> Result<B, E>>(initial: A, computation: F) -> B {
    loop {
        match computation(&initial) {
            Err(_) => continue,
            Ok(value) => return value,
        }
    }
}
