pub mod components;
pub mod helper;

use yew::html;
use yew::prelude::*;

use crate::components::pages::page_container::PageContainer;


use game::model::fastgen::generate;
use game::model::coordinate::Coordinate;
use crate::components::map::board::BoardComponent;
use crate::helper::screen::Screen;

#[function_component(App)]
fn app() -> Html {

    let level_grid = generate(Coordinate {row : 5, column: 5}, 99);
    let screen = use_state(|| Screen::Title);
    let msg = use_state(|| "".to_string());

    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>
            
            // <PageContainer />
            <div id="container">
                <BoardComponent 
                    level_grid={level_grid}
                    screen={screen.clone()}
                    message={msg.clone()}
                >
                </BoardComponent>
            </div>


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
