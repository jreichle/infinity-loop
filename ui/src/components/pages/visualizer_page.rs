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

use crate::components::board::level::StatelessLevelComponent;
use crate::components::utils::{slider::SliderComponent, tile_selector::TileSelector};
use crate::helper::local_storage::change_screen;
use crate::helper::screen::Screen;

type GameGrid = SentinelGrid<EnumSet<Tile<Square>>>;
type Weights = EnumMap<Tile<Square>, usize>;

const LOG_PREFIX: &str = "#viz";
const DEFAULT_WIDTH: isize = 10;
const DEFAULT_HEIGHT: isize = 10;
const DEFAULT_SPEED: isize = 80;

const PASS_LIMIT: usize = 40000;
const PROP_LIMIT: usize = 1000;

fn init_board(wfc_generator: &WfcGenerator) -> (GameGrid, Weights) {
    let (sentinel_grid, weights) = wfc_generator.init_board();
    wfc_generator.iteration_step(sentinel_grid, weights)
}

fn get_next_step(
    wfc_generator: WfcGenerator,
    sentinel_grid: GameGrid,
    weights: Weights,
) -> (GameGrid, Weights) {
    let (mut new_grid, mut new_weights) = (sentinel_grid, weights);
    if WfcGenerator::is_all_collapsed(&new_grid) {
        (new_grid, new_weights) = wfc_generator.init_board();
    }
    wfc_generator.iteration_step(new_grid, new_weights)
}


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

    let wfc_generator =
        WfcGenerator::with_all_tiles(DEFAULT_WIDTH as usize, DEFAULT_HEIGHT as usize);
    let (sentinel_grid, weights) = init_board(&wfc_generator);

    let available_tiles: UseStateHandle<EnumSet<Tile<Square>>> = use_state_eq(|| EnumSet::FULL);
    let wfc_generator = use_state_eq(|| wfc_generator);
    let sentinel_grid = use_state_eq(|| sentinel_grid);
    let weights = use_state_eq(|| weights);
    let level_grid = use_state_eq(|| WfcGenerator::extract_grid(&sentinel_grid));

    let update_onclick: Callback<MouseEvent> = {
        let available_tiles = available_tiles.clone();
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
            let new_generator =
                WfcGenerator::new(width, height, *available_tiles, PASS_LIMIT, PROP_LIMIT);
            let (new_grid, new_weights) = init_board(&new_generator);
            level_grid.set(WfcGenerator::extract_grid(&new_grid));
            wfc_generator.set(new_generator);
            sentinel_grid.set(new_grid);
            weights.set(new_weights);
        })
    };

    let next_onclick: Callback<MouseEvent> = {
        let wfc_generator = wfc_generator.clone();
        let level_grid = level_grid.clone();
        let sentinel_grid = sentinel_grid.clone();
        let weights = weights.clone();
        Callback::from(move |_| {
            log::debug!("{LOG_PREFIX} [Button click] next");
            let (new_grid, new_weights) = get_next_step(
                (*wfc_generator).clone(),
                (*sentinel_grid).clone(),
                (*weights).clone(),
            );
            level_grid.set(WfcGenerator::extract_grid(&new_grid));
            sentinel_grid.set(new_grid);
            weights.set(new_weights);
        })
    };

    let play_onclick: Callback<MouseEvent> = {
        let interval_id = interval_id;
        let wfc_generator = wfc_generator;
        let level_grid = level_grid.clone();
        let sentinel_grid = sentinel_grid;
        let weights = weights;

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

                let (new_grid, new_weights) = get_next_step(
                    (*wfc_generator).clone(),
                    (*sentinel_grid).clone(),
                    (*weights).clone(),
                );

                {
                    let wfc_generator = wfc_generator.clone();
                    let level_grid = level_grid.clone();
                    let sentinel_grid = sentinel_grid.clone();
                    let weights = weights.clone();
                    let mut new_grid = new_grid;
                    let mut new_weights = new_weights;

                    let iteration_closure = Closure::<dyn FnMut()>::new(move || {
                        (new_grid, new_weights) = get_next_step(
                            (*wfc_generator).clone(),
                            new_grid.clone(),
                            new_weights.clone(),
                        );
                        level_grid.set(WfcGenerator::extract_grid(&new_grid));
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

    let to_title: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::debug!("{LOG_PREFIX} [Button click] back - go back to Menu page");
            change_screen(screen.clone(), Screen::Title);
        })
    };

    html! {
        <div class="container">
            <div class="controller">
                <div class="selector-controller">
                    <TileSelector tile_set={available_tiles.clone()} />
                </div>
                <div class="flex-col">
                    <SliderComponent id="slider-height" label="#row" value={height_value.clone()} />
                    <SliderComponent id="slider-width" label="#col"  value={width_value.clone()} />
                </div>
                <button onclick={update_onclick.clone()}>
                    {"-update-"}
                </button>
            </div>
            <div class="game-board">
                <StatelessLevelComponent overlay_message={use_state_eq(|| String::from(""))} level_grid={(*level_grid).clone()} />
            </div>
            <div class="controller space-between">
                <div class="flex-col">
                    <SliderComponent id="slider-speed" label="#speed" value={speed_value.clone()} max=100 min=1 />
                    <button
                        onclick={play_onclick.clone()}
                    >
                    {
                        if *playing {"-pause-"} else {"-play-"}
                    }
                    </button>
                    <button
                        onclick={next_onclick.clone()}
                    >
                        {"-next-"}
                    </button>
                </div>

                <div class="flex-col">
                    <button
                        onclick={to_title.clone()}
                    >
                    {"-home-"}
                    </button>
                </div>

            </div>
        </div>
    }
}
