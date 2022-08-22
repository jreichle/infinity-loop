use yew::prelude::*;

use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;

use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;
use game::model::gameboard::GameBoard;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

pub enum MapAction {
    TurnCell(Coordinate<isize>),
    NextLevel,
    GetHint,
    SolveLevel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapState {
    pub level_number: usize,
    pub level_size: Coordinate<usize>,
    pub level_grid: Grid<Tile<Square>>,
}

impl Default for MapState {
    fn default() -> Self {
        Self {
            level_number: 1,
            level_size: Coordinate { row: 5, column: 5 },
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

impl Reducible for MapState {
    type Action = MapAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_level_grid: Grid<Tile<Square>> = self.level_grid.clone();
        let mut new_level_size: Coordinate<usize> = self.level_size;
        let mut new_level_number: usize = self.level_number.clone();

        match action {
            MapAction::TurnCell(index) => {
                new_level_grid = new_level_grid.rotate_clockwise(index).unwrap();
            }
            MapAction::NextLevel => {
                new_level_number += 1;
                new_level_size = Coordinate {
                    row: new_level_size.row + 1,
                    column: new_level_size.column + 1,
                };
                new_level_grid = generate(new_level_size, new_level_number as u64)
            }
            MapAction::SolveLevel => {
                if let Some(solution) = self.level_grid.solve().next() {
                    new_level_grid = solution;
                }
            }
            MapAction::GetHint => {
                log::info!("Get hint.");
                highlight_cells(3, 3);
            }
        };

        Self {
            level_number: new_level_number,
            level_size: new_level_size,
            level_grid: new_level_grid.clone(),
        }
        .into()
    }
}

impl MapState {
    pub fn set(grid: Grid<Tile<Square>>) -> impl Fn() -> MapState {
        move || MapState {
            level_number: 1,
            level_size: grid.dimensions(),
            level_grid: grid.clone(),
        }
    }
}
