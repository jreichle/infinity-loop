use yew::prelude::*;
use yew::{html, Callback};

use game::model::{
    coordinate::Coordinate,
    gameboard::GameBoard,
    grid::Grid,
    tile::{Square, Tile},
};

use crate::components::board::{cell::CellComponent, grid::GridComponent, row::RowComponent};
use crate::components::reducers::board_reducer::{BoardAction, Level};
use crate::helper::local_storage::save_level;

#[derive(Properties, PartialEq, Clone)]
pub struct LevelProps {
    pub board: UseReducerHandle<Level<Grid<Tile<Square>>>>,
    pub head_message: UseStateHandle<String>,
    #[prop_or(use_state_eq(|| "".to_string()))]
    pub overlay_message: UseStateHandle<String>,
    #[prop_or(true)]
    pub can_complete: bool,
    #[prop_or(false)]
    pub can_turn: bool,
    #[prop_or(false)]
    pub can_change: bool,
}

/// A playable level supporting mouse actions.
/// The state is managed by the board reducer.
#[function_component(LevelComponent)]
pub fn level_component(props: &LevelProps) -> html {
    fn dispatch_turn_cell(
        level: UseReducerHandle<Level<Grid<Tile<Square>>>>,
        index: Coordinate<isize>,
        can_change: bool,
        head_message: UseStateHandle<String>,
    ) -> Callback<MouseEvent> {
        Callback::from(move |_| {
            log::debug!(
                "Tile with coordinate {:?} has been clicked.",
                index.to_tuple()
            );
            log::debug!("can change? {}", can_change);
            if can_change || !level.data.is_solved() {
                level.dispatch(BoardAction::TurnCell(index));
                save_level(&level.data);
                log::debug!("saving level now");
            } else {
                head_message.set(String::from("The level is already solved"));
            }
        })
    }

    fn dispatch_change_cell(
        board: UseReducerHandle<Level<Grid<Tile<Square>>>>,
        index: Coordinate<isize>,
    ) -> Callback<WheelEvent> {
        Callback::from(move |_| {
            log::debug!(
                "Tile with coordinate {:?} has been wheeled.",
                index.to_tuple()
            );
            board.dispatch(BoardAction::ChangeTileShape(index));
            save_level(&board.data);
            log::debug!("saving level now");
        })
    }

    let board = props.board.clone();
    let level_grid = board.data.clone();
    let (height, width) = level_grid.dimensions().to_tuple();
    let (height, width) = (height as isize, width as isize);

    if props.can_complete {
        let overlay_message = props.overlay_message.clone();
        if !board.data.is_solved() {
            overlay_message.set(String::from(""));
        } else {
            overlay_message.set(String::from("-LEVEL COMPLETED-"));
        }
    }

    html! {
        <div class="game-board">
            <GridComponent overlay_message={props.overlay_message.clone()}>
                {
                    (0..height).into_iter().map(| row | {
                        html!{
                            <RowComponent key={row}>
                                {
                                    (0..width).into_iter().map(| column | {
                                        let index = Coordinate { row, column };
                                        let tile = *level_grid.get(index).unwrap();
                                        html!{
                                            <CellComponent
                                                key={column}
                                                tile={tile}
                                                row_number={row}
                                                column_number={column}
                                                on_click={
                                                    if props.can_turn {
                                                        dispatch_turn_cell(
                                                            board.clone(),
                                                            index,
                                                            props.can_change,
                                                            props.head_message.clone()
                                                        )
                                                    } else {
                                                        Callback::from(|_|{})
                                                    }
                                                }
                                                on_wheel={
                                                    if props.can_change {
                                                        dispatch_change_cell(board.clone(), index)
                                                    } else {
                                                        Callback::from(|_|{})
                                                    }
                                                }
                                            ></CellComponent>
                                        }
                                    }).collect::<Html>()
                                }
                            </RowComponent>
                        }
                    }).collect::<Html>()
                }
            </GridComponent>
        </div>
    }
}


/// A stateless level board that doesn't support any mouse action. 
/// Mainly used for preview.
#[derive(Properties, PartialEq, Clone)]
pub struct StatelessLevelProps {
    pub level_grid: Grid<Tile<Square>>,
    #[prop_or(use_state_eq(|| "".to_string()))]
    pub overlay_message: UseStateHandle<String>,
}

#[function_component(StatelessLevelComponent)]
pub fn stateless_level_component(props: &StatelessLevelProps) -> html {
    let level_grid = props.level_grid.clone();
    let (height, width) = level_grid.dimensions().to_tuple();
    let (height, width) = (height as isize, width as isize);

    html! {
        <div class="flex-col">
            <GridComponent overlay_message={props.overlay_message.clone()}>
                {
                    (0..height).into_iter().map(| row | {
                        html!{
                            <RowComponent key={row}>
                                {
                                    (0..width).into_iter().map(| column | {
                                        let index = Coordinate { row, column };
                                        let tile = *level_grid.get(index).unwrap();
                                        html!{
                                            <CellComponent
                                                key={column}
                                                tile={tile}
                                                row_number={row}
                                                column_number={column}
                                            ></CellComponent>
                                        }
                                    }).collect::<Html>()
                                }
                            </RowComponent>
                        }
                    }).collect::<Html>()
                }
            </GridComponent>
        </div>
    }
}
