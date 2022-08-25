use yew::html;
use yew::prelude::*;

mod components;
use components::editor::editor::EditorComponent;
use components::map::board::BoardComponent;
use components::map_preview::level_preview::LevelPreviewComponent;

mod helper;
use helper::screen::Screen;

use game::model::coordinate::Coordinate;

#[function_component(App)]
fn app() -> Html {
    let dimension = use_state(|| Coordinate::new(5 as usize, 5 as usize));
    let level_number = use_state(|| 0);
    let screen = use_state(|| Screen::Title);

    let to_preview: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.clone().set(Screen::Overview);
        })
    };

    let to_editor: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.clone().set(Screen::Editor);
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
                                    {"-Preview Levels-"}
                                </button>
                                <button onclick={to_editor}>
                                    {"-Level Editor-"}
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
