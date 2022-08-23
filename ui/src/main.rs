use yew::html;
use yew::prelude::*;

mod components;
use components::editor::editor::EditorComponent;
use components::map::board::BoardComponent;
use components::map_preview::level_preview::LevelPreviewComponent;

mod helper;
use helper::screen::Screen;

use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;
use game::model::grid::Grid;

#[function_component(App)]
fn app() -> Html {
    let dimension = use_state(|| Coordinate::new(5 as usize, 5 as usize));
    let level_number = use_state(|| 0);
    let grid_map = generate(*dimension, 1);
    let screen = use_state(|| Screen::Title);

    let to_preview: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.clone().set(Screen::Overview);
        })
    };

    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>

            {
                match &*screen {
                    Screen::Title => {
                        html! {
                            <div id="container">
                                <button onclick={to_preview}>
                                    {"Preview Levels"}
                                </button>
                            </div>
                        }
                    },
                    Screen::Overview => {
                        html! {
                            <div id="container">
                                <LevelPreviewComponent
                                    screen={screen.clone()}
                                    dimension={dimension}
                                    level_number={level_number}
                                />
                            </div>
                        }
                    },
                    Screen::Editor => {
                        html! {
                            <div id="container">
                                <EditorComponent screen={screen.clone()}/>
                            </div>
                        }
                    },
                    Screen::Level(user_grid) => {
                        html! {
                            <div id="container">
                                <BoardComponent
                                    level_grid={user_grid.clone()}
                                    screen={screen.clone()}/>
                            </div>
                        }
                    },
                    Screen::Solving => {
                        html! {
                            <div id="container">
                                <div>
                                    {"Solving Level ..."}
                                </div>
                            </div>
                        }
                    }

                }
            }
            <div id="footer">
                <a href={"https://uni2work.ifi.lmu.de/course/S22/IfI/Rust"}>
                    {"High level languages: Rust"}
                </a>
                {" - Group IV"}
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
