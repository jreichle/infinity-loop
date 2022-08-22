use yew::prelude::*;
use yew::{html, Properties};

use super::cell::CellComponent;
use super::board_reducer::BoardState;

#[derive(Properties, PartialEq, Clone)]
pub struct RowComponentProps {
    pub board_state: UseReducerHandle<BoardState>,
    pub row_number: isize,
}

#[function_component(RowComponent)]
pub fn row_component(props: &RowComponentProps) -> Html {
    let (_, width) = props.board_state.level_size.to_tuple();

    html! {
        <div class="cell-row">
            {
                (0..width).into_iter().map(| column_number | {
                    html!{
                        <CellComponent
                            key={column_number}
                            row_number={props.row_number.clone()}
                            column_number={column_number as isize}
                            map_state={props.map_state.clone()}
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
