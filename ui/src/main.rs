use yew::html;
use yew::prelude::*;

mod components;
use components::map::map::MapComponent;
use components::map_preview::level_preview::LevelPreviewComponent;

mod helper;
use helper::screen::Screen;

use game::generator::levelstream::{level_stream, LevelProperty};
use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;

#[function_component(App)]
fn app() -> Html {
    let screen = use_state(|| Screen::Title);
    let to_overview: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.clone().set(Screen::Overview);
        })
    };

    let grid_map = generate(Coordinate::new(5, 5), 1);

    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>
            if *screen == Screen::Title {
                <div id="container">
                    <button onclick={to_overview}>{"Start"}</button>
                </div>
            }
            if *screen == Screen::Overview {
                <div id="container">
                    <LevelPreviewComponent
                        level_count=20
                        screen={screen.clone()}
                    />
                </div>
            }
            if *screen == Screen::Level {
                <div id="container">
                    <MapComponent grid_map={grid_map} />
                </div>
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
