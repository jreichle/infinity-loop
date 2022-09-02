#![allow(dead_code)]

use rand::Rng;
use std::ops::Add;
use std::rc::Rc;
use yew::prelude::*;

use crate::helper::level_randomizer::randomize_level;
use crate::helper::local_storage::save_level;
use wasm_bindgen::{prelude::*, JsCast};

use game::generator::wfc::WfcGenerator;
use game::model::hint::generate_hint;
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

    // Editor actions
    ChangeTileShape(Coordinate<isize>),
    ChangeSize(Coordinate<usize>),
    GenerateFastGen,
    GenerateWFC,
    ShuffleTileRotations,
    ClearGrid,
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

impl Reducible for BoardState {
    type Action = BoardAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_level_number: usize = self.level_number;
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
                if let Ok(coordinate) = generate_hint(&new_level_grid) {
                    highlight_cells(
                        coordinate.row.try_into().unwrap(),
                        coordinate.column.try_into().unwrap(),
                    );
                    log::info!("Highlighting: {}", coordinate);
                }
            }
            BoardAction::SolveLevel => {
                let mut solved_versions = new_level_grid.solve();
                if let Some(solved_level) = solved_versions.next() {
                    log::info!("solved level:\n {}", solved_level);
                    save_level(&solved_level);
                    new_level_grid = solved_level;
                }
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
                while generation_result.is_err() {
                    generation_result = wfc.generate();
                }

                new_level_grid = generation_result.unwrap();
                log::info!("Generated grid\n{}", new_level_grid.to_string());
            }
            BoardAction::ShuffleTileRotations => {
                new_level_grid = randomize_level(new_level_grid);
                save_level(&new_level_grid);
                log::info!("Tile rotations shuffled\n{}", new_level_grid.to_string());
            }
            BoardAction::ClearGrid => {
                new_level_grid = Grid::new(
                    new_level_grid.dimensions(),
                    vec![Tile::NO_CONNECTIONS; new_level_grid.elements().len()],
                )
            }
        };

        Self {
            level_number: new_level_number,
            level_grid: new_level_grid,
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
