use yew::prelude::*;
use yew::{html, Html, Children};

use crate::components::map::board_reducer::BoardState;
use crate::components::map::row::RowComponent;

#[derive(Properties, PartialEq, Clone)]
pub struct GridComponentProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(GridComponent)]
pub fn grid_component(props: &GridComponentProps) -> Html {
    html! {
        <>
            <div class="game-board">
                { for props.children.iter() }
            </div>
        </>
    }
}
