// use components::map::MapLayout;
use yew::html;
use yew::prelude::*;

mod components;
use components::map::map::MapComponent;

mod helper;
use helper::screen::Screen;

use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;

#[function_component(App)]
fn app() -> Html {
    let grid_map = generate(Coordinate { row: 5, column: 5 }, 99);

    let screen = use_state(|| Screen::Title);

    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>
            if screen == Screen::Title {
                <button>{"Start"}</button>
            }
            <div id="container">
                <MapComponent grid_map={grid_map} />
            </div>
            <div id="footer"><a href={"https://uni2work.ifi.lmu.de/course/S22/IfI/Rust"}>{"High level languages: Rust"}</a>{" - Group IV"}</div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    log::info!("frontend starting...");
}
