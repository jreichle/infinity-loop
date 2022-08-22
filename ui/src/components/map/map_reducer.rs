use game::model::gameboard::GameBoard;
use std::rc::Rc;
use yew::prelude::*;

use game::model::coordinate::Coordinate;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

use game::model::fastgen::generate;

// reducer's action
pub enum MapAction {
    TurnCell(Coordinate<isize>),
    ChangeTileShape(Coordinate<isize>),
    NextLevel,
    SolveLevel,
}

// reducer's state
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapState {
    pub level_number: usize,
    pub level_size: Coordinate<usize>,
    pub level_grid: Grid<Tile<Square>>,
    pub allow_tile_change: bool,
}

impl Default for MapState {
    fn default() -> Self {
        Self {
            level_number: 1,
            level_size: Coordinate { row: 5, column: 5 },
            level_grid: generate(Coordinate { row: 5, column: 5 }, 1),
            allow_tile_change: false
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
            MapAction::ChangeTileShape(index) => {
                if self.allow_tile_change {
                    new_level_grid = new_level_grid.change_tile_shape(index).unwrap();
                }
            },
        };

        Self {
            level_number: new_level_number,
            level_size: new_level_size,
            level_grid: new_level_grid.clone(),
            allow_tile_change: self.allow_tile_change
        }
        .into()
    }
}

impl MapState {
    pub fn set(grid: Grid<Tile<Square>>, allow_tile_change: bool) -> impl Fn() -> MapState {
        move || MapState {
            level_number: 1,
            level_size: grid.dimensions(),
            level_grid: grid.clone(),
            allow_tile_change: allow_tile_change
        }
    }

    pub fn new(grid: Grid<Tile<Square>>, allow_tile_change: bool) -> MapState {
        MapState::set(grid, allow_tile_change)()
    }
}
