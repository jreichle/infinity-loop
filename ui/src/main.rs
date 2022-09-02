pub mod components;
pub mod helper;

use yew::html;
use yew::prelude::*;

use crate::components::pages::page_router::PageRouter;

use crate::components::utils::tile_selector::TileSelector;
use game::model::{
    enumset::EnumSet,
    tile::{Tile, Square}
};


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>
            <PageRouter />
            // <TileSelector tile_set={tile_set} />

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
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<App>();
}
