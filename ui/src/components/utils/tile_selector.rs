use yew::html;
use yew::prelude::*;

use game::{tile, enumset};
use game::model::{
    enumset::EnumSet,
    tile::{Tile, Square::{self, Up, Down, Left, Right}}
};

use crate::components::utils::tile_checkbox::TileCheckbox;

fn get_all_roations(tile: Tile<Square>) -> EnumSet<Tile<Square>> {
    // let rotations: EnumSet<Tile<Square>> = 
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
    pub state: UseStateHandle<bool>
}

impl TileState {
    pub fn new(tile: Tile<Square>, is_used: bool) -> Self {
        Self { tile, state: use_state_eq(|| is_used) }
    }
}


#[derive(Properties, PartialEq, Clone)]
pub struct TileSelectorProps {
    pub tile_set: UseStateHandle<EnumSet<Tile<Square>>>,
}

#[function_component(TileSelector)]
pub fn tile_selector_component(props: &TileSelectorProps) -> Html {
    let tile_set = props.tile_set.clone();

    let empty_tile = TileState::new(tile!(), false);
    let u_tile = TileState::new(tile!(Up), false);
    let ud_tile = TileState::new(tile!(Up, Down), false);
    let ur_tile = TileState::new(tile!(Up, Right), false);
    let urd_tile = TileState::new(tile!(Up, Right, Down), false);
    let urdl_tile = TileState::new(tile!(Up, Right, Down, Left), false);

    let tiles = vec![empty_tile.clone(), u_tile.clone(), ud_tile.clone(), ur_tile.clone(), urd_tile.clone(), urdl_tile.clone()];

    let a = tiles.iter()
        .filter(| tile | *(tile.state.clone()) )
        .map(| tile | tile.tile)
        .fold(EnumSet::EMPTY, | acc, x | acc.union(get_all_roations(x)));

    log::info!("{:?}", a.to_string());

    html!{
        <div class="tile-selector flex-col" style="width: 5vw;">
            <div class="flex-row">
                <TileCheckbox tile={empty_tile.tile} is_used={empty_tile.state} />
                <TileCheckbox tile={u_tile.tile} is_used={u_tile.state} />
                </div>
            <div class="flex-row">
                <TileCheckbox tile={ud_tile.tile} is_used={ud_tile.state} />
                <TileCheckbox tile={ur_tile.tile} is_used={ur_tile.state} />         
            </div>
            <div class="flex-row">
                <TileCheckbox tile={urd_tile.tile} is_used={urd_tile.state} />
                <TileCheckbox tile={urdl_tile.tile} is_used={urdl_tile.state} />                               
            </div>            
        </div>
    }
}