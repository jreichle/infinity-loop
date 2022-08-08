use yew::prelude::*;
use yew::html;

mod components;
use components::map::{
    MapComponent,
    MapComponentProps,
    RowComponentProps,
    CellComponentProps,
    get_index
};

#[function_component(App)]
fn app() -> Html {
    let level_data = "LLLTL\nTTT+T\nLL LT\nLTLIT\nLTILT\n-TTL-";
    let level_lines = level_data.lines().collect::<Vec<_>>();
    let props = MapComponentProps { 
            id: 1,
            children: level_lines.iter().map( | line | {
            RowComponentProps {
                children: line.clone().chars().map(| char | {
                    CellComponentProps { value: get_index(char) }
                }).collect()
            }
        } ).collect()
    };



    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>
            <div id="container">  
                <MapComponent ..props />
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    log::info!("frontend starting...");
}
