pub mod components;
pub mod helper;

use yew::html;
use yew::prelude::*;

use crate::components::pages::page_router::PageContainer;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>
            <PageContainer />
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
