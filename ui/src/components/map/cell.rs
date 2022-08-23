use yew::prelude::*;
use yew::{html, Callback, Properties};

use game::model::coordinate::Coordinate;
use crate::components::map::board_reducer::{BoardAction, BoardState};

#[derive(Properties, PartialEq, Clone)]
pub struct CellComponentProps {
    pub board_state: UseReducerHandle<BoardState>,
    pub row_number: isize,
    pub column_number: isize,
}

#[function_component(CellComponent)]
pub fn cell_component(props: &CellComponentProps) -> Html {
    let (row, column) = (props.row_number.clone(), props.column_number.clone());
    let index = Coordinate { row, column };

    let board_state = props.board_state.clone();
    let cell_tile = board_state.level_grid.get(index.clone()).unwrap();
    let cell_symbol = cell_tile.to_string().chars().next().unwrap();
    let cell_img = get_index(cell_symbol.clone());

    let img_path = vec![
        "data/tiles/0.svg",
        "data/tiles/1.svg",
        "data/tiles/2.svg",
        "data/tiles/3.svg",
        "data/tiles/4.svg",
        "data/tiles/5.svg",
    ];

    let board = board_state.clone();
    let onclick = Callback::from(move |e:MouseEvent| {
        log::info!(
            "Tile {} with coordinate ({}, {}) has been clicked.",
            cell_symbol,
            row,
            column
        );
        board.dispatch(BoardAction::TurnCell(index));
    });

    html! {
        <div id={format!("cell-r-{}-c-{}", row, column)} class={format!("cell row-{} col-{}", row, column)}>
            <img src={img_path[cell_img]}
                onclick={onclick}
                style={format!("{}{}{}","transform:rotate(", get_angle(cell_symbol), "deg);")}
            />
        </div>
    }
}

pub fn get_angle(cell_symbol: char) -> usize {
    match cell_symbol {
        ' ' | '╋' | '╹' | '┗' | '┣' => 0,
        '╺' | '━' | '┏' | '┳' => 90,
        '╻' | '┓' | '┫' => 180,
        '╸' | '┛' | '┻' => 270,
        _ => 0,
    }
}

pub fn get_index(cell_symbol: char) -> usize {
    match cell_symbol {
        ' ' => 0,
        '╹' | '╺' | '╻' | '╸' => 1,
        '┃' | '━' => 2,
        '┗' | '┏' | '┛' | '┓' => 3,
        '┣' | '┻' | '┫' | '┳' => 4,
        '╋' => 5,
        _ => 0,
    }
}