use yew::prelude::*;
use yew::{html, Callback};

use rand::Rng;

use super::level::LevelComponent;
use super::preview_reducer::{PreviewAction, PreviewState};

use crate::helper::screen::Screen;

use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;
use game::model::grid::Grid;
use game::model::testlevel::{parse_level, unicode_to_tile};
use game::model::tile::{Square, Tile};

#[derive(Properties, PartialEq, Clone)]
pub struct LevelPreviewComponentProps {
    pub screen: UseStateHandle<Screen>,
    pub dimension: UseStateHandle<Coordinate<usize>>,
    pub level_number: UseStateHandle<usize>,
}

#[function_component(LevelPreviewComponent)]
pub fn level_preview_component(props: &LevelPreviewComponentProps) -> Html {
    let generate_nr = 20;
    let mut generated_levels = (0..generate_nr)
        .into_iter()
        .map(|index| generate(*props.dimension, index as u64))
        .collect::<Vec<Grid<Tile<Square>>>>();

    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let nr_saved_levels = local_storage.length().unwrap();

    let mut saved_levels = (0..nr_saved_levels)
        .into_iter()
        .map(|index| {
            local_storage
                .get_item(format!("Own level {}", index + 1).as_str())
                .unwrap()
        })
        .map(|level| {
            if let Some(level_str) = level {
                parse_level(level_str.as_str(), unicode_to_tile).unwrap()
            } else {
                Grid::EMPTY
            }
        })
        .collect::<Vec<Grid<Tile<Square>>>>();

    let level_count = use_state(|| generate_nr + saved_levels.len());
    log::info!("lvls: {}", *level_count);
    generated_levels.append(&mut saved_levels);
    let reducer = use_reducer(PreviewState::set(generated_levels));

    let load_more_levels: Callback<MouseEvent> = {
        let reducer = reducer.clone();
        let level_count = level_count.clone();
        let dimension = props.dimension.clone();
        Callback::from(move |_| {
            log::info!("loading more levles");
            reducer.dispatch(PreviewAction::LoadNew(10, *dimension));
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
            screen.set(Screen::Level(reducer.extracted_levels[num].clone()));
        })
    };

    let create_own_level: Callback<MouseEvent> = {
        // let level_count = level_count.clone();
        let screen = props.screen.clone();
        // let reducer = reducer.clone();
        Callback::from(move |_| {
            log::info!("picking random level");
            // let num = rand::thread_rng().gen_range(0..*level_count);
            screen.set(Screen::Editor);
        })
    };

    let back: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Editor");
            screen.set(Screen::Title);
        })
    };

    html! {
        <>
            <div id="preview-container">
                {
                    (0..*level_count).into_iter().map( | level_index | {
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
            <div id="controller">
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
                    onclick={back}>
                    {"-back-"}
                </button>
            </div>
        </>
    }
}
