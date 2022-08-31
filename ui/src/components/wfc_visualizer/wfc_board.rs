use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, Callback};

use game::generator::wfc::WfcGenerator;
use game::model::{
    enummap::EnumMap,
    enumset::EnumSet,
    solver::SentinelGrid,
    tile::{Square, Tile},
};

// use std::collections::HashMap;
use super::level::LevelComponent;
use super::slider::SliderComponent;
use crate::helper::screen::Screen;

#[derive(Properties, PartialEq, Clone)]
pub struct WfcBoardComponentProps {
    pub screen: UseStateHandle<Screen>,
}

#[function_component(WfcBoardComponent)]
pub fn wfc_board_component(props: &WfcBoardComponentProps) -> Html {
    let (width_ref, height_ref) = (use_node_ref(), use_node_ref());
    let playing = use_state(|| false);
    let interval_id = use_state(|| 0);

    let wfc_generator = WfcGenerator::default(14, 14);
    let (sentinel_grid, weights) = wfc_generator.init_board();
    let (sentinel_grid, weights) = wfc_generator.iteration_step(sentinel_grid, weights);

    let wfc_generator = use_state_eq(|| wfc_generator);
    let sentinel_grid = use_state_eq(|| sentinel_grid);
    let weights = use_state_eq(|| weights);
    let level_grid = use_state_eq(|| WfcGenerator::extract_grid(&sentinel_grid));

    fn go_to_next_step(
        wfc_generator: WfcGenerator,
        sentinel_grid: SentinelGrid<EnumSet<Tile<Square>>>,
        weights: EnumMap<Tile<Square>, usize>,
    ) -> (
        SentinelGrid<EnumSet<Tile<Square>>>,
        EnumMap<Tile<Square>, usize>,
    ) {
        let (mut new_grid, mut new_weights) = (sentinel_grid, weights);
        if WfcGenerator::is_all_collapsed(&new_grid) {
            (new_grid, new_weights) = wfc_generator.init_board();
        }
        wfc_generator.iteration_step(new_grid, new_weights)
    }

    let new_onclick: Callback<MouseEvent> = {
        let wfc_generator = wfc_generator.clone();
        let level_grid = level_grid.clone();
        let sentinel_grid = sentinel_grid.clone();
        let weights = weights.clone();
        let width_input_ref = width_ref.clone();
        let height_input_ref = height_ref.clone();
        Callback::from(move |_| {
            if let (Some(width_input), Some(height_input)) = (
                width_input_ref.cast::<HtmlInputElement>(),
                height_input_ref.cast::<HtmlInputElement>(),
            ) {
                let (width, height) = (
                    width_input.value_as_number() as usize,
                    height_input.value_as_number() as usize,
                );
                log::info!("new grid size: ({}, {})", width, height);
                let new_generator = WfcGenerator::default(width, height);
                let (mut new_grid, mut new_weights) = new_generator.init_board();
                (new_grid, new_weights) = new_generator.iteration_step(new_grid, new_weights);
                level_grid.set(WfcGenerator::extract_grid(&new_grid));
                wfc_generator.set(new_generator);
                sentinel_grid.set(new_grid);
                weights.set(new_weights);
            }
        })
    };

    let next_onclick: Callback<MouseEvent> = {
        let wfc_generator = wfc_generator.clone();
        let level_grid = level_grid.clone();
        let sentinel_grid = sentinel_grid.clone();
        let weights = weights.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Next.");
            let (new_grid, new_weights) = go_to_next_step(
                (*wfc_generator).clone(),
                (*sentinel_grid).clone(),
                (*weights).clone(),
            );
            level_grid.set(WfcGenerator::extract_grid(&new_grid));
            sentinel_grid.set(new_grid.clone());
            weights.set(new_weights.clone());
        })
    };

    let play_onclick: Callback<MouseEvent> = {
        let interval_id = interval_id.clone();
        let wfc_generator = wfc_generator.clone();
        let level_grid = level_grid.clone();
        let sentinel_grid = sentinel_grid.clone();
        let weights = weights.clone();

        let playing = playing.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Play.");
            if !(*playing) {
                log::info!("Start playing...");
                playing.set(true);

                let (new_grid, new_weights) = go_to_next_step(
                    (*wfc_generator).clone(),
                    (*sentinel_grid).clone(),
                    (*weights).clone(),
                );

                {
                    let wfc_generator = wfc_generator.clone();
                    let level_grid = level_grid.clone();
                    let sentinel_grid = sentinel_grid.clone();
                    let weights = weights.clone();
                    let mut new_grid = new_grid.clone();
                    let mut new_weights = new_weights.clone();

                    let iteration_closure = Closure::<dyn FnMut()>::new(move || {
                        (new_grid, new_weights) = go_to_next_step(
                            (*wfc_generator).clone(),
                            new_grid.clone(),
                            new_weights.clone(),
                        );
                        level_grid.set(WfcGenerator::extract_grid(&new_grid));
                        sentinel_grid.set(new_grid.clone());
                        weights.set(new_weights.clone());
                    });

                    let window = web_sys::window().unwrap();
                    let id = window
                        .set_interval_with_callback_and_timeout_and_arguments_0(
                            iteration_closure.as_ref().unchecked_ref(),
                            5,
                        )
                        .ok()
                        .unwrap();

                    interval_id.set(id);
                    iteration_closure.forget();
                }
            } else {
                log::info!("Already playing...");
            }
        })
    };

    let stop_onclick: Callback<MouseEvent> = {
        let interval_id = interval_id.clone();
        let playing = playing.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Stop.");
            if *playing {
                log::info!("Stop playing...");
                playing.set(false);
                web_sys::window()
                    .unwrap()
                    .clear_interval_with_handle(*interval_id);
            } else {
                log::info!("not playing...");
            }
        })
    };

    let back_onclick: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Editor");
            screen.set(Screen::Title);
        })
    };

    html! {
        <div id="container">
            <LevelComponent level_grid={(*level_grid).clone()} />
            <div id="controller">
                <SliderComponent id="slider-height" label="#row" node_ref={height_ref} />
                <SliderComponent id="slider-width" label="#col"  node_ref={width_ref} />

                <button
                    onclick={new_onclick}
                >
                    {"-new-"}
                </button>
                <button
                    onclick={next_onclick}
                >
                    {"-next-"}
                </button>
                <button
                    onclick={play_onclick}
                >
                {"-play-"}
                </button>
                <button
                    onclick={stop_onclick}
                >
                {"-stop-"}
                </button>
                <button
                    onclick={back_onclick}
                >
                {"-back-"}
                </button>


            </div>
        </div>
    }
}
