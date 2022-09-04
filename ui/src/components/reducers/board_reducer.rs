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
    finite::Finite,
};



// reducer's action
pub enum BoardAction {
    TurnCell(Coordinate<isize>),
    ReplaceGrid(Grid<Tile<Square>>),
    NextLevel,
    GetHint,
    SolveLevel,
    SolveLevelInput(String),
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

pub fn check_match(first: Tile<Square>, second: Tile<Square>) -> bool{
    match first.enum_to_index(){
        0 => second.enum_to_index() == 0,
        1 => second.enum_to_index() == 1 || second.enum_to_index() == 2 || second.enum_to_index() == 4 || second.enum_to_index() == 8,
        2 => second.enum_to_index() == 1 || second.enum_to_index() == 2 || second.enum_to_index() == 4 || second.enum_to_index() == 8,
        4 => second.enum_to_index() == 1 || second.enum_to_index() == 2 || second.enum_to_index() == 4 || second.enum_to_index() == 8,
        8 => second.enum_to_index() == 1 || second.enum_to_index() == 2 || second.enum_to_index() == 4 || second.enum_to_index() == 8,

        3 => second.enum_to_index() == 3 || second.enum_to_index() == 6 || second.enum_to_index() == 9 || second.enum_to_index() == 12,
        6 => second.enum_to_index() == 3 || second.enum_to_index() == 6 || second.enum_to_index() == 9 || second.enum_to_index() == 12,
        9 => second.enum_to_index() == 3 || second.enum_to_index() == 6 || second.enum_to_index() == 9 || second.enum_to_index() == 12,
        12 => second.enum_to_index() == 3 || second.enum_to_index() == 6 || second.enum_to_index() == 9 || second.enum_to_index() == 12,

        5 => second.enum_to_index() == 5 || second.enum_to_index() == 10 ,
        10 => second.enum_to_index() == 5 || second.enum_to_index() == 10 ,

        7 => second.enum_to_index() == 7 || second.enum_to_index() == 11 || second.enum_to_index() == 13 || second.enum_to_index() == 14,
        11 => second.enum_to_index() == 7 || second.enum_to_index() == 11 || second.enum_to_index() == 13 || second.enum_to_index() == 14,
        13 => second.enum_to_index() == 7 || second.enum_to_index() == 11 || second.enum_to_index() == 13 || second.enum_to_index() == 14,
        14 => second.enum_to_index() == 7 || second.enum_to_index() == 11 || second.enum_to_index() == 13 || second.enum_to_index() == 14,

        15 => second.enum_to_index() == 15,

        _ => false
    }
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
            BoardAction::SolveLevelInput(input) => {               
                let mut solved_version = new_level_grid.solve_with_input(&input);
                    log::info!("solved level:\n {}", solved_version);

                    let mut same = true;

                    let first_grid = solved_version.as_slice().clone();
                    let second_grid = new_level_grid.as_slice().clone();

                    for i in 0..first_grid.len() {
                        if !check_match(first_grid[i], second_grid[i]) {
                            same = false;
                        }
                    }
                    if solved_version.is_solved() && same {
                        new_level_grid = solved_version;
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
