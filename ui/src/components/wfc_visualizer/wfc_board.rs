use wasm_bindgen::{prelude::*, JsCast};
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
    let (width_value, height_value, speed_value) = (
        use_state(|| 14_isize),
        use_state(|| 14_isize),
        use_state(|| 80_isize),
    );
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
        let width_value = width_value.clone();
        let height_value = height_value.clone();
        Callback::from(move |_| {
            let (width, height) = (*width_value as usize, *height_value as usize);
            log::info!("new grid size: ({}, {})", width, height);
            let new_generator = WfcGenerator::default(width, height);
            let (mut new_grid, mut new_weights) = new_generator.init_board();
            (new_grid, new_weights) = new_generator.iteration_step(new_grid, new_weights);
            level_grid.set(WfcGenerator::extract_grid(&new_grid));
            wfc_generator.set(new_generator);
            sentinel_grid.set(new_grid);
            weights.set(new_weights);
        })
    };

    // TODO:
    // MOVE the setInterval outside of the Callback

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
        let speed_value = speed_value.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Play.");
            if *playing {
                log::info!("Stop playing...");
                playing.set(false);
                web_sys::window()
                    .unwrap()
                    .clear_interval_with_handle(*interval_id);
            } else {
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

                    let speed = 10 * (100 - *speed_value as i32);
                    let window = web_sys::window().unwrap();    
                    let id = window
                        .set_interval_with_callback_and_timeout_and_arguments_0(
                            iteration_closure.as_ref().unchecked_ref(),
                            speed,
                        )
                        .ok()
                        .unwrap();

                

                    interval_id.set(id);
                    iteration_closure.forget();
                }
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
        <>
            <LevelComponent level_grid={(*level_grid).clone()} />
            <div id="controller">
                <div class="flex-col margin-bot-4vh">
                    <SliderComponent id="slider-height" label="#row" value={height_value.clone()} />
                    <SliderComponent id="slider-width" label="#col"  value={width_value.clone()} />
                    <button
                        onclick={new_onclick}
                    >
                        {"-resize-"}
                    </button>
                </div>

                <div class="flex-col margin-bot-4vh">
                    <SliderComponent id="slider-speed" label="#speed" value={speed_value.clone()} max=100 min=1 />
                    <button
                        onclick={play_onclick}
                    >
                    {
                        if *playing {"-pause-"} else {"-play-"}
                    }
                    </button>
                    <button
                        onclick={next_onclick}
                    >
                        {"-next-"}
                    </button>           
                </div>     

                <div class="flex-col margin-bot-4vh">
                    <button
                        onclick={back_onclick}
                    >
                    {"-back-"}
                    </button>
                </div>

            </div>
        </>
    }
}
