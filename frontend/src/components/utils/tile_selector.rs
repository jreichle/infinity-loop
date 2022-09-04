use yew::html;
use yew::prelude::*;

use game::model::tile::{
        Square::{self, Down, Left, Right, Up},
        Tile,
};

use game::core::enumset::EnumSet;
use game::{enumset, tile};

use crate::components::utils::tile_checkbox::TileCheckbox;

/// get all possiblities for given tile
fn get_all_roations(tile: Tile<Square>) -> EnumSet<Tile<Square>> {
    enumset!(
        tile,
        tile.rotated_clockwise(1),
        tile.rotated_clockwise(2),
        tile.rotated_clockwise(3)
    )
}

#[derive(Debug, Clone)]
struct TileState {
    pub tile: Tile<Square>,
    pub state: UseStateHandle<bool>,
}

impl TileState {
    pub fn new(tile: Tile<Square>, is_used: bool) -> Self {
        Self {
            tile,
            state: use_state_eq(|| is_used),
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TileSelectorProps {
    #[prop_or(use_state_eq(|| EnumSet::FULL))]
    pub tile_set: UseStateHandle<EnumSet<Tile<Square>>>,
}

/// A selector with checkbox of all tile shapes. Keeps track of the tile set.
#[function_component(TileSelector)]
pub fn tile_selector_component(props: &TileSelectorProps) -> Html {
    let tile_set = props.tile_set.clone();
    log::debug!("tile_set: {}", tile_set.to_string());

    // all tile shapes base on connections (c)
    let tile_0c = TileState::new(tile!(), true);
    let tile_1c = TileState::new(tile!(Up), true);
    let tile_2c_line = TileState::new(tile!(Up, Down), true);
    let tile_2c_turn = TileState::new(tile!(Up, Right), true);
    let tile_3c = TileState::new(tile!(Up, Right, Down), true);
    let tile_4c = TileState::new(tile!(Up, Right, Down, Left), true);

    let tiles = vec![
        tile_0c.clone(),
        tile_1c.clone(),
        tile_2c_line.clone(),
        tile_2c_turn.clone(),
        tile_3c.clone(),
        tile_4c.clone(),
    ];

    let new_tile_set = tiles
        .iter()
        .filter(|tile| *(tile.state.clone()))
        .map(|tile| tile.tile)
        .fold(EnumSet::EMPTY, |acc, x| acc.union(get_all_roations(x)));

    tile_set.set(new_tile_set);

    html! {
        <div class="tile-selector flex-col">
            <div class="flex-row">
                <TileCheckbox tile={tile_0c.tile} is_used={tile_0c.state} />
                <TileCheckbox tile={tile_1c.tile} is_used={tile_1c.state} />
                </div>
            <div class="flex-row">
                <TileCheckbox tile={tile_2c_line.tile} is_used={tile_2c_line.state} />
                <TileCheckbox tile={tile_2c_turn.tile} is_used={tile_2c_turn.state} />
            </div>
            <div class="flex-row">
                <TileCheckbox tile={tile_3c.tile} is_used={tile_3c.state} />
                <TileCheckbox tile={tile_4c.tile} is_used={tile_4c.state} />
            </div>
        </div>
    }
}
