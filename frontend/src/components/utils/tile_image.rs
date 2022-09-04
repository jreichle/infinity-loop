use yew::html;
use yew::prelude::*;

use game::model::tile::{Square, Tile};

const IMG_PATH: [&str; 6] = [
    "data/tiles/0.svg",
    "data/tiles/1.svg",
    "data/tiles/2.svg",
    "data/tiles/3.svg",
    "data/tiles/4.svg",
    "data/tiles/5.svg",
];

#[derive(Properties, PartialEq, Clone)]
pub struct TileImageProps {
    pub tile: Tile<Square>,
}

/// image representation of tile in the game model
#[function_component(TileImage)]
pub fn tile_image_component(props: &TileImageProps) -> Html {
    let tile = props.tile;
    let cell_symbol = tile.to_string().chars().next().unwrap();
    let cell_img = get_index(cell_symbol);

    html! {
        <div class="tile">
            <img src={IMG_PATH[cell_img]}
                style={format!("{}{}{}",
                    "transform:rotate(",
                    get_angle(cell_symbol),
                    "deg);")}
            />
        </div>
    }
}

/// get the correct image for given tile
fn get_index(cell_symbol: char) -> usize {
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


/// get the correct angle for image roation 
fn get_angle(cell_symbol: char) -> usize {
    match cell_symbol {
        ' ' | '╋' | '╹' | '┗' | '┣' => 0,
        '╺' | '━' | '┏' | '┳' => 90,
        '╻' | '┓' | '┫' => 180,
        '╸' | '┛' | '┻' => 270,
        _ => 0,
    }
}

