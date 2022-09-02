use yew::prelude::*;
use yew::{html, Callback};

use rand::Rng;

use crate::components::board::level::StatelessLevelComponent;
use crate::components::reducers::preview_reducer::{PreviewAction, PreviewState};

use crate::helper::local_storage::{
    change_screen, retrieve_editor_level, retrieve_preview_level_count, save_preview_level_count,
};
use crate::helper::screen::Screen;

use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

#[derive(Properties, PartialEq, Clone)]
pub struct LevelPreviewPageProps {
    pub screen: UseStateHandle<Screen>,
    pub dimension: UseStateHandle<Coordinate<usize>>,
    pub level_number: UseStateHandle<usize>,
}

#[function_component(LevelPreviewPage)]
pub fn level_preview_page_component(props: &LevelPreviewPageProps) -> Html {
    let generate_nr = retrieve_preview_level_count();
    let level_count = use_state(|| generate_nr);
    let generated_levels = (0..generate_nr)
        .into_iter()
        .map(|index| generate(*props.dimension, index as u64))
        .collect::<Vec<Grid<Tile<Square>>>>();

    let saved_level = retrieve_editor_level();

    let reducer = use_reducer(PreviewState::set(generated_levels));

    let load_more_levels: Callback<MouseEvent> = {
        let reducer = reducer.clone();
        let level_count = level_count.clone();
        let dimension = props.dimension.clone();
        Callback::from(move |_| {
            log::info!("loading more levles");
            reducer.dispatch(PreviewAction::LoadNew(10, *dimension));
            save_preview_level_count(*level_count + 10);
            level_count.set(*level_count + 10);
        })
    };

    let pick_random_level: Callback<MouseEvent> = {
        let level_count = level_count.clone();
        let screen = props.screen.clone();
        let reducer = reducer.clone();
        Callback::from(move |_| {
            log::info!("picking random level");
            let num = rand::thread_rng().gen_range(0..*level_count);
            let level = reducer.extracted_levels[num].clone();
            change_screen(screen.clone(), Screen::Level(level));
        })
    };

    let create_own_level: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::info!("picking random level");
            change_screen(screen.clone(), Screen::Editor);
        })
    };

    let to_title: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Editor");
            change_screen(screen.clone(), Screen::Title);
        })
    };

    fn to_level_action(
        level_grid: Grid<Tile<Square>>,
        screen: UseStateHandle<Screen>,
    ) -> Callback<MouseEvent> {
        Callback::from(move |_| {
            change_screen(screen.clone(), Screen::Level(level_grid.clone()));
        })
    }

    html! {
        <>
            if saved_level != Grid::EMPTY {
                <div id="saved-level-container">
                    <div id="saved-level">
                        <div
                            class="level-container"
                            onclick={to_level_action(
                                saved_level.clone(),
                                props.screen.clone())}>
                            <StatelessLevelComponent level_grid={saved_level.clone()} />
                            <div class="level-title">{"Saved"}</div>
                        </div>
                    </div>
                </div>
            }
            <div id="container">
                <div id="preview-container">
                    {
                        (0..*level_count).into_iter().map( | level_index | {
                            let level_grid = reducer.extracted_levels[level_index].clone();
                            html!{
                                <div
                                    class="level-container"
                                    onclick={to_level_action(
                                        saved_level.clone(),
                                        props.screen.clone())}>
                                    <StatelessLevelComponent level_grid={level_grid.clone()} />
                                    <div class="level-title">
                                        {format!("#{}", level_index + 1)}
                                    </div>
                            </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="controller">
                    <button
                        onclick={load_more_levels}>
                        {"-load more-"}
                    </button>
                    <button
                        onclick={pick_random_level}>
                        {"-pick random-"}
                    </button>
                    <button
                        onclick={create_own_level}>
                        {"-create your own-"}
                    </button>
                    <button
                        onclick={to_title}>
                        {"-back to start-"}
                    </button>
                </div>
            </div>
        </>
    }
}
