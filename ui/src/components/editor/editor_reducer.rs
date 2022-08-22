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

use super::super::map::map_reducer::MapState;

#[derive(Clone, Debug, PartialEq)]
pub struct EditorState
{
    pub map: MapState
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
            map: MapState::default()
        }
    }
}

impl Reducible for EditorState {
    type Action = EditorAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_map = self.map.clone();

        match action {
            EditorAction::TurnCell(index) => {
                new_map.level_grid = new_map.level_grid.rotate_clockwise(index).unwrap();
            }
            EditorAction::ChangeSize(size) => {
                new_map.level_grid.resize(size);
            },
            EditorAction::GenerateFastGen => {
                let mut rng = rand::thread_rng();
                new_map.level_grid = fastgen::generate(new_map.level_size, rng.gen_range(0..10000));
                log::info!("Generated grid\n{}", new_map.level_grid.to_string());
            },
            EditorAction::GenerateWFC =>{
                let wfc = WfcGenerator::new(new_map.level_size.column,
                    new_map.level_size.row, 
                    Tile::ALL_CONNECTIONS.0, 
                    40000, 
                    1000);

                let mut generation_result = wfc.generate();
                while let Err(_) = generation_result {
                    generation_result = wfc.generate();
                }

                new_map.level_grid = generation_result.unwrap();
                log::info!("Generated grid\n{}", new_map.level_grid.to_string());
            },
        };

        Self {
            map: new_map
        }
        .into()
    }
}

impl EditorState {
    pub fn set(map: MapState) -> impl Fn() -> EditorState {
        move || EditorState {
            map: map.clone()
        }
    }
}
