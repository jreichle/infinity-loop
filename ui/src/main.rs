use yew::html;
use yew::prelude::*;

mod components;
// use components::map_preview::level_preview::LevelPreviewComponent;
use components::editor::editor::EditorComponent;
use components::map::board::BoardComponent;
// use components::wfc_visualizer::live_map::WfcVisualizerComponent;

mod helper;
use helper::screen::Screen;
use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;


#[function_component(App)]
fn app() -> Html {
    let grid_map = generate(Coordinate { row: 5, column: 5 }, 99);
    let screen = use_state(|| Screen::Title);
    let to_overview: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.clone().set(Screen::Overview);
        })
    };
    let to_level: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.set(Screen::Level(generate(Coordinate { row: 5, column: 5 }, 99)));
        })
    };
    let to_editor: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.set(Screen::Editor);
        })
    };


    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>

            {
                match &*screen {
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
                    _ => {
                        html! {
                            <div id="container">
                                <BoardComponent 
                                    level_grid={generate(Coordinate { row: 5, column: 5 }, 99)} 
                                    screen={screen.clone()}/>
                            </div>
                        }
                    },
                }
            }

            <div id="footer">
                <a href={"https://uni2work.ifi.lmu.de/course/S22/IfI/Rust"}>
                    {"High level languages: Rust"}
                </a>{" - Group IV"}
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    log::info!("frontend starting...");
}
