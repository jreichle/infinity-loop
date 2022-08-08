/*
use game::model::coordinate::Coordinate;
use game::model::gameboard::GameBoard;
use game::model::grid::Grid;
use game::model::tile::Square;
use game::model::tile::Tile;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct BoardProperties {
    pub board: Grid<Tile<Square>>,
}

#[function_component(board)]
pub fn generate_game_board_html(prop: &BoardProperties) -> Html {
    let board = &prop.board;
    let pos_to_tile = board.serialize_board();
    let rows = extract_res_or_default(board.rows().try_into(), 0);
    let cols = extract_res_or_default(board.columns().try_into(), 0);
    let current_row: isize = 0;
    let current_col: isize = 0;

    while current_row < rows {
        while current_col < cols {
            println!(
                "{:?}",
                pos_to_tile.get(&Coordinate {
                    row: current_row,
                    column: current_col
                })
            )
        }
    }
    html! {
        <div>
            {"aa"}
        </div>
    }
}

fn extract_res_or_default<T, E>(res: Result<T, E>, default: T) -> T {
    match res {
        Ok(ret) => ret,
        Err(err) => default,
    }
}
*/
