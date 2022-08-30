use std::rc::Rc;
use yew::prelude::*;

use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

use crate::helper::level_randomizer::randomize_level;

pub enum PreviewAction {
    LoadNew(usize, Coordinate<usize>),
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
            PreviewAction::LoadNew(num, dimension) => {
                let mut generated_levels = (0..num)
                    .into_iter()
                    .map(|index| randomize_level(generate(dimension, index as u64)))
                    .collect::<Vec<Grid<Tile<Square>>>>();
                extracted_levels.append(&mut generated_levels);
            }
        }

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
