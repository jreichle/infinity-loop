use yew::prelude::*;
use yew::{html, Children};

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
