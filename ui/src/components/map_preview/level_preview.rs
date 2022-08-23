use yew::prelude::*;
use yew::{html, Html};

use super::level::LevelComponent;
use super::preview_reducer::PreviewState;
use crate::helper::screen::Screen;
use game::generator::levelstream::{level_stream, LevelProperty};
use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

// needs to contain
// - levelstream
//      - hook to level stream
//          - overview as to take multiple level
//          - if level in middle is picked -> level stream is missing levels in between
//          - maybe: if level in between was picked -> remaining into iterator -> chain
//      - int with picked level
//          -> level compoenent can setup level stream
//
// Add randomization to mapping

#[derive(Properties, PartialEq, Clone)]
pub struct LevelPreviewComponentProps {
    pub level_count: usize,
    pub screen: UseStateHandle<Screen>,
    pub dimension: UseStateHandle<Coordinate<usize>>,
    pub level_number: UseStateHandle<usize>,
}

#[function_component(LevelPreviewComponent)]
pub fn level_preview_component(props: &LevelPreviewComponentProps) -> Html {
    let generated_levels = (0..props.level_count)
        .into_iter()
        .map(|index| generate(*props.dimension, index as u64))
        .collect::<Vec<Grid<Tile<Square>>>>();

    let reducer = use_reducer(PreviewState::set(generated_levels, 0));

    // TODO: figure out which level
    // TODO: add form to change dimension

    html! {
        <div id="preview-container">
            {
                (0..props.level_count).into_iter().map( | level_index | {
                    html!{
                        <LevelComponent
                            preview_state={reducer.clone()}
                            screen={props.screen.clone()}
                            level_index={level_index}
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
