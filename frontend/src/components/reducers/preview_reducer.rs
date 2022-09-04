use std::rc::Rc;
use yew::prelude::*;

use game::generator::fastgen::generate;
use game::model::coordinate::Coordinate;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

use crate::helper::level_randomizer::randomize_level;

/// reducer facilitates loading more levels for the preview component
///
/// the LoadNew action takes a number which indicates the amount of levels to load and
/// a coordinate which indicates needed dimension of the levels
pub enum PreviewAction {
    LoadNew(usize, Coordinate<usize>),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PreviewState {
    pub extracted_levels: Vec<Grid<Tile<Square>>>,
}

impl Reducible for PreviewState {
    type Action = PreviewAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            PreviewAction::LoadNew(level_number, dimension) => {
                let generated_levels = (0..level_number as u64)
                    .into_iter()
                    .map(|index| randomize_level(generate(dimension, index)));
                Self {
                    extracted_levels: self
                        .extracted_levels
                        .clone()
                        .into_iter()
                        .chain(generated_levels)
                        .collect(),
                }
                .into()
            }
        }
    }
}

impl PreviewState {
    pub fn set(extracted_levels: Vec<Grid<Tile<Square>>>) -> impl Fn() -> PreviewState {
        move || PreviewState {
            extracted_levels: extracted_levels
                .clone()
                .into_iter()
                .map(randomize_level)
                .collect(),
        }
    }
}
