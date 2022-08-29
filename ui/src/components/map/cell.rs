use yew::prelude::*;
use yew::{html, Callback, Properties};

use game::model::{
    tile::{Tile, Square}
};

#[derive(Properties, PartialEq, Clone)]
pub struct CellComponentProps {
    pub tile: Tile<Square>,
    pub row_number: isize,
    pub column_number: isize,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_wheel: Callback<WheelEvent>,
}

#[function_component(CellComponent)]
pub fn cell_component(props: &CellComponentProps) -> Html {
    let (row, column) = (props.row_number.clone(), props.column_number.clone());
    let cell_tile = props.tile.clone();

    let cell_symbol = cell_tile.to_string().chars().next().unwrap();
    let cell_img = get_index(cell_symbol.clone());

    let img_path = vec![
        "data/tiles/0.svg",
        "data/tiles/1.svg",
        "data/tiles/2.svg",
        "data/tiles/3.svg",
        "data/tiles/4.svg",
        "data/tiles/5.svg",
    ];

    html! {
        <div
            id={format!("cell-r-{}-c-{}", row, column)}
            class={format!("cell row-{} col-{}", row, column)}>
            <img src={img_path[cell_img]}
                onclick={props.on_click.clone()}
                onwheel={props.on_wheel.clone()}
                style={format!("{}{}{}",
                    "transform:rotate(",
                    get_angle(cell_symbol),
                    "deg);")}
            />
        </div>
    }
}

pub fn get_angle(cell_symbol: char) -> usize {
    match cell_symbol {
        ' ' | '╋' | '╹' | '┗' | '┣' => 0,
        '╺' | '━' | '┏' | '┳' => 90,
        '╻' | '┓' | '┫' => 180,
        '╸' | '┛' | '┻' => 270,
        _ => 0,
    }
}

pub fn get_index(cell_symbol: char) -> usize {
    match cell_symbol {
        ' ' => 0,
        '╹' | '╺' | '╻' | '╸' => 1,
        '┃' | '━' => 2,
        '┗' | '┏' | '┛' | '┓' => 3,
        '┣' | '┻' | '┫' | '┳' => 4,
        '╋' => 5,
        _ => 0,
    }
}
