use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;
use yew::{html, Callback};

use game::generator::wfc::WfcSettings;
use game::model::{
    enummap::EnumMap,
    enumset::EnumSet,
    solver::SentinelGrid,
    tile::{Square, Tile},
};

use crate::components::board::level::StatelessLevelComponent;
use crate::components::utils::slider::SliderComponent;
use crate::helper::local_storage::change_screen;
use crate::helper::screen::Screen;

const LOG_PREFIX: &str = "#viz";
const DEFAULT_WIDTH: isize = 10;
const DEFAULT_HEIGHT: isize = 10;
const DEFAULT_SPEED: isize = 80;

#[derive(Properties, PartialEq, Clone)]
pub struct VisualizerPageProps {
    pub screen: UseStateHandle<Screen>,
}

#[function_component(VisualizerPage)]
pub fn wfc_board_component(props: &VisualizerPageProps) -> Html {
    let (width_value, height_value, speed_value) = (
        use_state(|| DEFAULT_WIDTH),
        use_state(|| DEFAULT_HEIGHT),
        use_state(|| DEFAULT_SPEED),
    );
    let playing = use_state(|| false);
    let interval_id = use_state(|| 0);

    let wfc_generator = WfcSettings::with_all_tiles(DEFAULT_WIDTH as usize, DEFAULT_HEIGHT as usize);
    let (sentinel_grid, weights) = wfc_generator.init_board();
    let (sentinel_grid, weights) = wfc_generator.iteration_step(sentinel_grid, weights);

    let wfc_generator = use_state_eq(|| wfc_generator);
    let sentinel_grid = use_state_eq(|| sentinel_grid);
    let weights = use_state_eq(|| weights);
    let level_grid = use_state_eq(|| WfcSettings::extract_grid(&sentinel_grid));

    fn go_to_next_step(
        wfc_generator: WfcSettings,
        sentinel_grid: SentinelGrid<EnumSet<Tile<Square>>>,
        weights: EnumMap<Tile<Square>, usize>,
    ) -> (
        SentinelGrid<EnumSet<Tile<Square>>>,
        EnumMap<Tile<Square>, usize>,
    ) {
        let (mut new_grid, mut new_weights) = (sentinel_grid, weights);
        if WfcSettings::is_all_collapsed(&new_grid) {
            (new_grid, new_weights) = wfc_generator.init_board();
        }
        wfc_generator.iteration_step(new_grid, new_weights)
    }

    let new_onclick: Callback<MouseEvent> = {
        let wfc_generator = wfc_generator.clone();
        let level_grid = level_grid.clone();
        let sentinel_grid = sentinel_grid.clone();
        let weights = weights.clone();
        let (width_value, height_value) = (width_value.clone(), height_value.clone());
        let (width, height) = (*width_value as usize, *height_value as usize);
        Callback::from(move |_| {
            log::debug!(
                "{LOG_PREFIX} [Button click] new: new grid generated with dimension: ({}, {})",
                width,
                height
            );
            let new_generator = WfcSettings::with_all_tiles(width, height);
            let (mut new_grid, mut new_weights) = new_generator.init_board();
            (new_grid, new_weights) = new_generator.iteration_step(new_grid, new_weights);
            level_grid.set(WfcSettings::extract_grid(&new_grid));
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
            log::debug!("{LOG_PREFIX} [Button click] next");
            let (new_grid, new_weights) = go_to_next_step(
                (*wfc_generator).clone(),
                (*sentinel_grid).clone(),
                (*weights).clone(),
            );
            level_grid.set(WfcSettings::extract_grid(&new_grid));
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
            if *playing {
                log::debug!("{LOG_PREFIX} [Button click] pause: interval has been cleared");
                playing.set(false);
                web_sys::window()
                    .unwrap()
                    .clear_interval_with_handle(*interval_id);
            } else {
                log::debug!("{LOG_PREFIX} [Button click] play: interval started");
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
                        level_grid.set(WfcSettings::extract_grid(&new_grid));
                        sentinel_grid.set(new_grid.clone());
                        weights.set(new_weights.clone());
                    });

                    let speed = 3 * (100 - *speed_value as i32);
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
            log::debug!("{LOG_PREFIX} [Button click] back - go back to Menu page");
            change_screen(screen.clone(), Screen::Title);
        })
    };

    html! {
        <div class="container">
            <div class="game-board">
            <StatelessLevelComponent level_grid={(*level_grid).clone()} />
            </div>
            <div class="controller">
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
        </div>
    }
}
