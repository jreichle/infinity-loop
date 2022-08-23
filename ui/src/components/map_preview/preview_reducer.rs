use rand::Rng;
use std::rc::Rc;
use yew::prelude::*;

use game::generator::levelstream::{level_stream, LevelProperty};
use game::model::coordinate::Coordinate;
use game::model::gameboard::GameBoard;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

pub enum PreviewAction {
    ChooseLevel(usize),
    Random,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewState {
    pub extracted_levels: Vec<Grid<Tile<Square>>>,
    pub chosen_level: usize,
}

impl Reducible for PreviewState {
    type Action = PreviewAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut extracted_levels = self.extracted_levels.clone();
        let mut chosen_level = self.chosen_level;

        match action {
            PreviewAction::ChooseLevel(index) => log::info!("choose level _"),
            PreviewAction::Random => log::info!("random"),
        };

        // TODO: make new_extracted_levels, new_chosen_level
        Self {
            extracted_levels: extracted_levels,
            chosen_level: chosen_level,
        }
        .into()
    }
}

impl Default for PreviewState {
    fn default() -> Self {
        Self {
            extracted_levels: Vec::new(),
            chosen_level: 0,
        }
    }
}

impl PreviewState {
    pub fn set(
        mut extracted_levels: Vec<Grid<Tile<Square>>>,
        chosen_level: usize,
    ) -> impl Fn() -> PreviewState {
        for i in 0..extracted_levels.len() {
            let mut level_grid = extracted_levels[i].clone();
            log::info!("before rand: \n {}", level_grid);
            let dimension = level_grid.dimensions();
            while level_grid.is_solved() {
                for row in 0..dimension.row {
                    for col in 0..dimension.column {
                        let num = rand::thread_rng().gen_range(0..3);
                        for _ in 0..num {
                            level_grid = level_grid
                                .rotate_clockwise(Coordinate::new(row as isize, col as isize))
                                .unwrap();
                        }
                    }
                }
            }

            log::info!("after rand: \n {}", level_grid);
            extracted_levels.remove(i);
            extracted_levels.insert(i, level_grid);
        }
        move || PreviewState {
            extracted_levels: extracted_levels.clone(),
            chosen_level,
        }
    }
}
