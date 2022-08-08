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

pub fn parse_level(level_data: &str) -> MapComponentProps {
    let level_lines = level_data.lines().collect::<Vec<_>>();
    MapComponentProps { 
            id: 1,
            children: level_lines.iter().enumerate().map( | (row, line) | {
            RowComponentProps {
                row_count: row,
                children: line.clone().chars().enumerate().map(| (column, char) | {
                    CellComponentProps { 
                        coordinate: (row, column),
                        value: get_index(char) 
                    }
                }).collect()
            }
        } ).collect()
    }
}

#[function_component(App)]
fn app() -> Html {

    let level_data = "LLLTL\nTTT+T\nLL LT\nLTLIT\nLTILT\n-TTL-";
    let props = parse_level(level_data);

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
