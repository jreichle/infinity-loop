use yew::html;
use yew::prelude::*;

use crate::components::utils::tile_image::TileImageComponent;

use game::{tile, enumset};
use game::model::{
    tile::{Tile, Square::{self, Up, Down, Left, Right}}
};

#[derive(Properties, PartialEq, Clone)]
pub struct TileSelectorProps {
    // pub tile: Tile<Square>,
}

#[function_component(TileSelectorComponent)]
pub fn tile_selector(props: &TileSelectorProps) -> Html {
    html!{
        <div class="tile-selector" style="display:flex; flex-direction: column; width: 200px; height: 200px;">
            <div style="display:flex; flex-direction: row;">
                <TileImageComponent tile={tile!()} />
                <TileImageComponent tile={tile!(Up)} />
                </div>
            <div style="display:flex; flex-direction: row;">
                <TileImageComponent tile={tile!(Up, Down)} />
                <TileImageComponent tile={tile!(Up, Right)} />
            </div>
            <div style="display:flex; flex-direction: row;">
                <TileImageComponent tile={tile!(Up, Right, Down)} />
                <TileImageComponent tile={tile!(Up, Right, Down, Left)} />
            </div>            
        </div>
    }
}