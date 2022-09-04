use yew::html;
use yew::prelude::*;

use crate::components::utils::tile_image::TileImage;
use game::model::tile::{Square, Tile};

#[derive(Properties, PartialEq, Clone)]
pub struct TileCheckboxProps {
    pub tile: Tile<Square>,
    #[prop_or(use_state(||false))]
    pub is_used: UseStateHandle<bool>,
}

/// A single checkbox with a tile image, select/unselect on click
#[function_component(TileCheckbox)]
pub fn tile_checkbox_component(props: &TileCheckboxProps) -> Html {
    let is_used = props.is_used.clone();
    let tile = props.tile;

    let on_click: Callback<MouseEvent> = {
        let is_used = is_used.clone();
        let is_used_value = *is_used;
        Callback::from(move |_| {
            is_used.set(!is_used_value);
        })
    };

    let mut id = tile
        .0
        .iter()
        .map(|dir| dir.to_string().to_lowercase())
        .collect::<Vec<String>>()
        .join("-");
    if id.is_empty() {
        id = "empty".to_string();
    }

    html! {
        <div id={format!("option-{id}")}
            class={classes!("tile-checkbox"
                ,{
                    if !*is_used {
                        Some("unchecked")
                    } else { None::<&str> }
                }
            )
            }
            onclick={on_click.clone()}
         >
            <TileImage tile={tile} />
        </div>
    }
}
