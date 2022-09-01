use yew::prelude::*;
use yew::{html, Callback, Properties};

use game::model::{
    tile::{Tile, Square}
};

use crate::components::utils::tile_image::TileImageComponent;

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

    html! {
        <div
            id={format!("cell-r-{}-c-{}", row, column)}
            class={format!("cell row-{} col-{}", row, column)}
            onclick={props.on_click.clone()}
            onwheel={props.on_wheel.clone()}
            >
            <TileImageComponent tile={cell_tile} />
        </div>
    }
}