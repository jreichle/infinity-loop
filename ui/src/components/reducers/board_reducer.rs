#![allow(dead_code)]

use std::ops::Add;
use std::rc::Rc;
use rand::Rng;
use yew::prelude::*;

use crate::helper::level_randomizer::randomize_level;
use crate::helper::local_storage::save_level;
use wasm_bindgen::{prelude::*, JsCast};

use game::generator::wfc::WfcGenerator;
use game::model::{
    coordinate::Coordinate,
    fastgen::generate,
    gameboard::GameBoard,
    grid::Grid,
    tile::{Square, Tile},
};


// reducer's action
pub enum BoardAction {
    TurnCell(Coordinate<isize>),
    ReplaceGrid(Grid<Tile<Square>>),
    NextLevel,
    GetHint,
    SolveLevel,
    SolveLevelInput,
    GenerateCnf,

    // Editor actions
    ChangeTileShape(Coordinate<isize>),
    ChangeSize(Coordinate<usize>),
    GenerateFastGen,
    GenerateWFC,
    ShuffleTileRotations,
    ClearGrid
}

// reducer's state
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardState {
    pub level_number: usize,
    pub level_grid: Grid<Tile<Square>>,
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            level_number: 1,
            level_grid: generate(Coordinate { row: 5, column: 5 }, 1),
        }
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
    let highlight_class_names = format!("{} {}", class_names.clone(), "cell-hint-highlight");
    cell.set_class_name(&highlight_class_names);
    let hl = Closure::<dyn Fn()>::new(move || {
        cell.set_class_name(&class_names);
    });

    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(hl.as_ref().unchecked_ref(), 500)
        .ok();
    hl.forget();
}

impl Reducible for BoardState {
    type Action = BoardAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_level_number: usize = self.level_number.clone();
        let mut new_level_grid: Grid<Tile<Square>> = self.level_grid.clone();

        match action {
            BoardAction::TurnCell(index) => {
                new_level_grid = new_level_grid.rotate_clockwise(index).unwrap();
            }
            BoardAction::ReplaceGrid(grid) => {
                new_level_grid = grid;
            }
            BoardAction::NextLevel => {
                new_level_number += 1;
                new_level_grid = randomize_level(generate(
                    self.level_grid.dimensions().add(1),
                    new_level_number as u64,
                ));
                save_level(&new_level_grid);
            }
            BoardAction::GetHint => {
                log::info!("Get hint.");
                highlight_cells(3, 3);
            }
            BoardAction::SolveLevel => {
                let mut solved_versions = new_level_grid.solve();
                if let Some(solved_level) = solved_versions.next() {
                    log::info!("solved level:\n {}", solved_level);
                    new_level_grid = solved_level;
                }
            }
            BoardAction::SolveLevelInput => {
                let input ="3 6 7 12 19 22 21 23 25 27 28 34 35 40 37 42 41 43 45 47 48 55 53 59 63 61 67 65 73 79 77 82 81 86 85 88 92 97 -13 -14 -15 -16 -29 -30 -31 -32 -49 -50 -51 -52 -69 -70 -71 -72 -93 -94 -95 -96 -1 -4 -24 -2 -8 -44 -62 -64 -68 -66 -83 -84 -5 -9 -11 -10 -26 -46 -87 -89 -91 -90 -36 -56 -54 -76 -20 -33 -38 -60 -74 -75 -80 -78 -100 -17 -18 -39 -57 -58 -98 -99";
                let mut solved_versions = new_level_grid.solve_with_input(input);
                if let Some(solved_level) = Some(solved_versions) {
                    log::info!("solved level:\n {}", solved_level);
                    new_level_grid = solved_level;
                }
            }

            BoardAction::GenerateCnf => {
                let mut solved_versions = new_level_grid.generate_cnf();
                // if let Some(solved_level) = Some(solved_versions) {
                //     log::info!("solved level:\n {}", solved_level);
                //     new_level_grid = solved_level;
                // }
            }

            // Editor actions
            BoardAction::ChangeTileShape(index) => {
                log::info!("Change tile shape");
                new_level_grid = new_level_grid.change_tile_shape(index).unwrap();
            }
            BoardAction::ChangeSize(size) => {
                let mut rng = rand::thread_rng();
                new_level_grid = generate(size, rng.gen_range(0..10000));
            }
            BoardAction::GenerateFastGen => {
                let mut rng = rand::thread_rng();
                new_level_grid = generate(self.level_grid.dimensions(), rng.gen_range(0..10000));
                log::info!("Generated grid\n{}", new_level_grid.to_string());
            }
            BoardAction::GenerateWFC => {
                let (height, width) = self.level_grid.dimensions().to_tuple();
                let wfc = WfcGenerator::new(
                    width as usize,
                    height as usize,
                    Tile::ALL_CONNECTIONS.0,
                    40000,
                    1000,
                );

                let mut generation_result = wfc.generate();
                while let Err(_) = generation_result {
                    generation_result = wfc.generate();
                }

                new_level_grid = generation_result.unwrap();
                log::info!("Generated grid\n{}", new_level_grid.to_string());
            }
            BoardAction::ShuffleTileRotations => {
                let mut rng = rand::thread_rng();
                for c in 0..new_level_grid.dimensions().column {
                    for r in 0..new_level_grid.dimensions().row {
                        new_level_grid = new_level_grid
                            .rotate_clockwise_n_times(
                                Coordinate {
                                    row: r as isize,
                                    column: c as isize,
                                },
                                rng.gen_range(0..4),
                            )
                            .unwrap();
                    }
                }
                log::info!("Tile rotations shuffled\n{}", new_level_grid.to_string());
            }
            BoardAction::ClearGrid => new_level_grid = Grid::new(new_level_grid.dimensions(), vec![Tile::NO_CONNECTIONS; new_level_grid.elements().len()]),

        };

        Self {
            level_number: new_level_number,
            level_grid: new_level_grid.clone(),
        }
        .into()
    }
}

impl BoardState {
    pub fn set_size(dimensions: Coordinate<usize>) -> impl Fn() -> BoardState {
        move || BoardState {
            level_number: 1,
            level_grid: generate(dimensions, 1),
        }
    }

    pub fn set_grid(grid: Grid<Tile<Square>>) -> impl Fn() -> BoardState {
        move || BoardState {
            level_number: 1,
            level_grid: grid.clone(),
        }
    }
}
