use game::model::gameboard::GameBoard;
use rand::Rng;
use std::rc::Rc;
use yew::prelude::*;

use game::model::coordinate::Coordinate;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

use game::generator::wfc::WfcGenerator;
use game::model::fastgen;

#[derive(Clone, Debug, PartialEq)]
pub struct EditorState {
    pub grid_size: Coordinate<usize>,
    pub grid: Grid<Tile<Square>>,
}

// reducer's action
pub enum EditorAction {
    TurnCell(Coordinate<isize>),
    ChangeTileShape(Coordinate<isize>),
    ChangeSize(Coordinate<usize>),
    GenerateFastGen,
    GenerateWFC,
    ShuffleTileRotations,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            grid_size: Coordinate { row: 5, column: 5 },
            grid: fastgen::generate(Coordinate { row: 5, column: 5 }, 1),
        }
    }
}

impl Reducible for EditorState {
    type Action = EditorAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_grid = self.grid.clone();

        match action {
            EditorAction::TurnCell(index) => {
                new_grid = new_grid.rotate_clockwise(index).unwrap();
            }
            EditorAction::ChangeTileShape(index) => {
                log::info!("Change tile shape");
                new_grid = new_grid.change_tile_shape(index).unwrap();
            }
            EditorAction::ChangeSize(size) => {
                let mut rng = rand::thread_rng();
                new_grid = fastgen::generate(size, rng.gen_range(0..10000));
            }
            EditorAction::GenerateFastGen => {
                let mut rng = rand::thread_rng();
                new_grid = fastgen::generate(self.grid_size, rng.gen_range(0..10000));
                log::info!("Generated grid\n{}", new_grid.to_string());
            }
            EditorAction::GenerateWFC => {
                let wfc = WfcGenerator::new(
                    self.grid_size.column,
                    self.grid_size.row,
                    Tile::ALL_CONNECTIONS.0,
                    40000,
                    1000,
                );

                let mut generation_result = wfc.generate();
                while let Err(_) = generation_result {
                    generation_result = wfc.generate();
                }

                new_grid = generation_result.unwrap();
                log::info!("Generated grid\n{}", new_grid.to_string());
            }
            EditorAction::ShuffleTileRotations => {
                let mut rng = rand::thread_rng();
                for c in 0..new_grid.dimensions().column {
                    for r in 0..new_grid.dimensions().row {
                        new_grid = new_grid
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
                log::info!("Tile rotations shuffled\n{}", new_grid.to_string());
            }
        };

        Self {
            grid_size: new_grid.dimensions(),
            grid: new_grid,
        }
        .into()
    }
}

impl EditorState {
    pub fn set(grid: Grid<Tile<Square>>) -> impl Fn() -> EditorState {
        move || EditorState {
            grid_size: grid.dimensions(),
            grid: grid.clone(),
        }
    }
}
