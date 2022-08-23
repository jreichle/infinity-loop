use game::model::gameboard::GameBoard;
use std::rc::Rc;
use rand::Rng;
use yew::prelude::*;
use yew::{html, Callback, Html, Properties};

use game::model::coordinate::Coordinate;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

use game::model::fastgen;
use game::generator::wfc::WfcGenerator;

use crate::components::map::board_reducer::BoardState;

#[derive(Clone, Debug, PartialEq)]
pub struct EditorState
{
    pub board: BoardState
}

// reducer's action
pub enum EditorAction {
    ChangeSize(Coordinate<usize>),
    TurnCell(Coordinate<isize>),
    GenerateFastGen,
    GenerateWFC
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            board: BoardState::default()
        }
    }
}

impl Reducible for EditorState {
    type Action = EditorAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_board = self.board.clone();

        match action {
            EditorAction::TurnCell(index) => {
                new_board.level_grid = new_board.level_grid.rotate_clockwise(index).unwrap();
            }
            EditorAction::ChangeSize(size) => {
                new_board.level_grid.resize(size);
            },
            EditorAction::GenerateFastGen => {
                let mut rng = rand::thread_rng();
                new_board.level_grid = fastgen::generate(new_board.level_size, rng.gen_range(0..10000));
                log::info!("Generated grid\n{}", new_board.level_grid.to_string());
            },
            EditorAction::GenerateWFC =>{
                let wfc = WfcGenerator::new(new_board.level_size.column,
                    new_board.level_size.row, 
                    Tile::ALL_CONNECTIONS.0, 
                    40000, 
                    1000);

                let mut generation_result = wfc.generate();
                while let Err(_) = generation_result {
                    generation_result = wfc.generate();
                }

                new_board.level_grid = generation_result.unwrap();
                log::info!("Generated grid\n{}", new_board.level_grid.to_string());
            },
        };

        Self {
            board: new_board
        }
        .into()
    }
}

impl EditorState {
    pub fn set(board: BoardState) -> impl Fn() -> EditorState {
        move || EditorState {
            board: board.clone()
        }
    }
}
