
use yew::prelude::*;
use yew::html;

use game::model::{
    tile::{Tile, Square}
};

pub fn get_img_from_tile(tile: &Tile<Square>) -> Html {
    let img_path = vec![
        "data/tiles/0.svg",
        "data/tiles/1.svg",
        "data/tiles/2.svg",
        "data/tiles/3.svg",
        "data/tiles/4.svg",
        "data/tiles/5.svg",
    ];

    let cell_symbol = tile.to_string().chars().next().unwrap();
    let cell_img = get_index(cell_symbol.clone());

    html!{
        <img src={img_path[cell_img]}
            style={format!("{}{}{}",
                "transform:rotate(",
                get_angle(cell_symbol),
                "deg);")}
        />
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