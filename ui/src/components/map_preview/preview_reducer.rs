use std::rc::Rc;
use yew::prelude::*;

use game::generator::levelstream::{level_stream, LevelProperty};
use game::model::coordinate::Coordinate;
use game::model::gameboard::GameBoard;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

use crate::helper::level_randomizer::randomize_level;

pub enum PreviewAction {
    Random,
    LoadNew(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewState {
    pub extracted_levels: Vec<Grid<Tile<Square>>>,
}

impl Reducible for PreviewState {
    type Action = PreviewAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut extracted_levels = self.extracted_levels.clone();

        match action {
            PreviewAction::Random => log::info!("random"),
            PreviewAction::LoadNew(num) => log::info!("load new levels"),
        };

        Self { extracted_levels }.into()
    }
}

impl Default for PreviewState {
    fn default() -> Self {
        Self {
            extracted_levels: Vec::new(),
        }
    }
}

impl PreviewState {
    pub fn set(mut extracted_levels: Vec<Grid<Tile<Square>>>) -> impl Fn() -> PreviewState {
        for i in 0..extracted_levels.len() {
            let new_level = randomize_level(extracted_levels[i].clone());
            extracted_levels.remove(i);
            extracted_levels.insert(i, new_level);
        }
        move || PreviewState {
            extracted_levels: extracted_levels.clone(),
        }
    }
}
