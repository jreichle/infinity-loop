use yew::prelude::*;
use yew::html;

mod components;
use components::map::{
    MapComponemt,
    MapComponemtProps,
    RowComponemtProps,
    CellComponemtProps,
    get_index
};

#[function_component(App)]
fn app() -> Html {
    let level_data = "LLLTL\nTTT+T\nLL LT\nLTLIT\nLTILT\n-TTL-";
    let level_lines = level_data.lines().collect::<Vec<_>>();
    let props = MapComponemtProps { 
            id: 1,
            children: level_lines.iter().map( | line | {
            RowComponemtProps {
                children: line.clone().chars().map(| char | {
                    CellComponemtProps { value: get_index(char) }
                }).collect()
            }
        } ).collect()
    };



    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>
            <div id="container">  
                <MapComponemt ..props />
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    log::info!("frontend starting...");
}
