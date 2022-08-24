use yew::prelude::*;
use yew::{html, Html};

use crate::components::map::board_reducer::BoardState;
use crate::components::map::row::RowComponent;

#[derive(Properties, PartialEq, Clone)]
pub struct GridComponentProps {
    pub board_state: UseReducerHandle<BoardState>,
}

#[function_component(GridComponent)]
pub fn grid_component(props: &GridComponentProps) -> Html {
    let board = props.board_state.clone();
    let level_grid = board.level_grid.clone();
    let (height, _) = level_grid.dimensions().to_tuple();

    html! {
        <>
            <div class="game-board">
                {
                    (0..height).into_iter().map(| row_number | {
                        html!{
                            <RowComponent
                                key={row_number}
                                row_number={row_number as isize}
                                board_state={board.clone()}
                            />
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}
