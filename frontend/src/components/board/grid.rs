use yew::prelude::*;
use yew::{html, Children};

#[derive(Properties, PartialEq, Clone)]
pub struct GridComponentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(use_state_eq(|| "".to_string()))]
    pub overlay_message: UseStateHandle<String>,
}

/// A complete representation of the grid in the game model.
/// Allows overlay messages to display over the grid if needed.
#[function_component(GridComponent)]
pub fn grid_component(props: &GridComponentProps) -> Html {
    let children = props.children.clone();
    let overlay_message = props.overlay_message.clone();

    html! {
        <>
            { for children.iter() }

            {
                if !overlay_message.trim().is_empty() {
                    html!{
                        <div class="level-overlay">
                            <div class="overlay-text">{ (*overlay_message).clone() }</div>
                        </div>
                    }
                } else { html!{} }
            }


        </>
    }
}
