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
            { for props.children.iter() }
        </>
    }
}
