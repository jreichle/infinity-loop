use yew::prelude::*;
use yew::{html, Html};

use crate::components::reducers::preview_reducer::PreviewState;
use crate::components::map::cell::{get_angle, get_index};

use crate::components::map::level::StatelessLevelComponent;

use crate::helper::screen::Screen;

use game::model::coordinate::Coordinate;

#[derive(Properties, PartialEq, Clone)]
pub struct LevelComponentProps {
    pub preview_state: UseReducerHandle<PreviewState>,
    pub screen: UseStateHandle<Screen>,
    pub level_index: usize,
}

#[function_component(LevelComponent)]
pub fn level_preview_component(props: &LevelComponentProps) -> Html {
    let level_grid = props.preview_state.extracted_levels[props.level_index].clone();
    let (height, width) = level_grid.dimensions().to_tuple();

    let to_level: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        let level = level_grid.clone();
        Callback::from(move |_| {
            screen.set(Screen::Level(level.clone()));
        })
    };

    html! {
        <div class="level-container" onclick={to_level}>
            <StatelessLevelComponent level_grid={level_grid.clone()} />
            <div class="level-title">{format!("#{}", props.level_index + 1)}</div>
        </div>
    }
}
