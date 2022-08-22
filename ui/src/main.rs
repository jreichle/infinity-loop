use yew::prelude::*;
use yew::html;

mod components;
// use components::map_preview::level_preview::LevelPreviewComponent;
use components::map::board::BoardComponent;
// use components::wfc_visualizer::live_map::WfcVisualizerComponent;

use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;

#[function_component(App)]
fn app() -> Html {
    let grid_map = generate(Coordinate { row: 5, column: 5 }, 99);
    
    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>
            <div id="container">  
                // <WfcVisualizerComponent grid_map={grid_map} />
                <BoardComponent level_grid={grid_map} />
                // <LevelPreviewComponent level_count=100 />
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
