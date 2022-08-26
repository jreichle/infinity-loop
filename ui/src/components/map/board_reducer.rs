#![allow(dead_code)]

use std::ops::Add;
use std::rc::Rc;
use yew::prelude::*;

use wasm_bindgen::{prelude::*, JsCast};
use crate::helper::level_randomizer::randomize_level;

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
