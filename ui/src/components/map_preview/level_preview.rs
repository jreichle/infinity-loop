use yew::prelude::*;
use yew::{html, Html};

use super::level::LevelComponent;
use super::preview_reducer::PreviewState;
use crate::helper::screen::Screen;
use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

#[derive(Properties, PartialEq, Clone)]
pub struct LevelPreviewComponentProps {
    pub screen: UseStateHandle<Screen>,
    pub dimension: UseStateHandle<Coordinate<usize>>,
    pub level_number: UseStateHandle<usize>,
}

#[function_component(LevelPreviewComponent)]
pub fn level_preview_component(props: &LevelPreviewComponentProps) -> Html {
    let level_count = 20;
    let generated_levels = (0..level_count)
        .into_iter()
        .map(|index| generate(*props.dimension, index as u64))
        .collect::<Vec<Grid<Tile<Square>>>>();

    let reducer = use_reducer(PreviewState::set(generated_levels));

    // TODO: add form to change dimension

    html! {
        <div id="preview-container">
            {
                (0..level_count).into_iter().map( | level_index | {
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
