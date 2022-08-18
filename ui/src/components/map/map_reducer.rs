use game::model::gameboard::GameBoard;
use yew::prelude::*;
use std::rc::Rc;

use game::model::coordinate::Coordinate;
use game::model::grid::Grid;
use game::model::tile::{Tile, Square};

use game::model::fastgen::generate;

// reducer's action
pub enum MapAction {
    TurnCell(Coordinate<isize>),
    NextLevel,
    SolveLevel,
}

// reducer's state
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

impl Reducible for MapState {
    type Action = MapAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {

        let mut new_level_grid: Grid<Tile<Square>> = self.level_grid.clone();
        let mut new_level_size: Coordinate<usize> = self.level_size;
        let mut new_level_number: usize = self.level_number.clone();

        match action {
            MapAction::TurnCell(index) => { 
                new_level_grid = new_level_grid.rotate_clockwise(index).unwrap(); 
            },
            MapAction::NextLevel => {
                new_level_number += 1;
                new_level_size = Coordinate { row: new_level_size.row + 1, column: new_level_size.column + 1 };
                new_level_grid = generate(new_level_size, new_level_number as u64)
            },
            MapAction::SolveLevel => {},
        };

        Self {
            level_number: new_level_number,
            level_size: new_level_size,
            level_grid: new_level_grid.clone(),
        }.into()
    }
}

impl MapState {
    pub fn set(grid: Grid<Tile<Square>>) -> impl Fn() -> MapState {
        move | | MapState { level_number: 1, level_size: grid.dimensions(), level_grid: grid.clone() }
    }
}