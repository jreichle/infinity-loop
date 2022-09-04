use yew::prelude::*;
use yew::{html, Children, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct RowComponentProps {
    pub children: Children,
}

/// A row in the level board, which is filled with cells.
#[function_component(RowComponent)]
pub fn row_component(props: &RowComponentProps) -> Html {
    html! {
        <div class="cell-row">
            { for props.children.iter() }
        </div>
    }
}
