use yew::html;
use yew::prelude::*;

use crate::components::map::level::StatelessLevelComponent;
use crate::components::reducers::preview_reducer::PreviewState;
use crate::helper::local_storage::change_screen;
use crate::helper::screen::Screen;

#[derive(Properties, PartialEq, Clone)]
pub struct LevelComponentProps {
    pub preview_state: UseReducerHandle<PreviewState>,
    pub screen: UseStateHandle<Screen>,
    pub level_index: usize,
}

#[function_component(LevelComponent)]
pub fn level_preview_component(props: &LevelComponentProps) -> Html {
    let level_grid = props.preview_state.extracted_levels[props.level_index].clone();

    let to_level: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        let level = level_grid.clone();
        Callback::from(move |_| {
            change_screen(screen.clone(), Screen::Level(level.clone()));
        })
    };

    html! {
        <div class="level-container" onclick={to_level}>
            <StatelessLevelComponent level_grid={level_grid.clone()} />
            <div class="level-title">{format!("#{}", props.level_index + 1)}</div>
        </div>
    }
}
