use game::model::tile::Square;
use yew::prelude::*;
use yew::{html, Html};

use game::model::{
    grid::Grid,
    tile::Tile,
};

use crate::components::map::row::RowComponent;

use super::board_reducer::BoardState;

#[derive(Properties, PartialEq, Clone)]
pub struct GridComponentProps {
    pub board_state: UseReducerHandle<BoardState>,
}

#[function_component(GridComponent)]
pub fn grid_component(props: &GridComponentProps) -> Html {   
    
    let board = props.board_state.clone();

    // -- FROM grid -> is_solved
    // let row_slice = |r| {
    //     (0..width)
    //         .map(|c| Coordinate { row: r, column: c })
    //         .collect::<Vec<_>>()
    // };

    // let column_slice = |c| {
    //     (0..height)
    //         .map(|r| Coordinate { row: r, column: c })
    //         .collect::<Vec<_>>()
    // };

    // TODO: only pass row state & cell state instead of the whole game state

    let level_grid = board.level_grid.clone();
    let (height, _) = level_grid.dimensions().to_tuple();

    html! {
        <>
            <div class="game-board">
                {
                    (0..height).into_iter().map(| row_number | {
                        html!{ 
                            <RowComponent key={row_number} row_number={row_number as isize} board_state={board.clone()} /> 
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}