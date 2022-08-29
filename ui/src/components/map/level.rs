use yew::prelude::*;
use yew::{html, Callback};

use game::model::{
    grid::Grid,
    coordinate::Coordinate,
    tile::{Square, Tile},
};

use crate::components::map::{
    row::RowComponent,
    cell::CellComponent,
    board_reducer::{BoardAction, BoardState},
    grid::GridComponent,
};


#[derive(Properties, PartialEq, Clone)]
pub struct LevelProps {
    pub board: UseReducerHandle<BoardState>,
    #[prop_or(false)]
    pub can_turn: bool,
    #[prop_or(false)]
    pub can_change: bool,
}

#[function_component(LevelComponent)]
pub fn level_component(props: &LevelProps) -> html {

    fn dispatch_turn_cell(board: UseReducerHandle<BoardState>, index: Coordinate<isize>) -> Callback<MouseEvent> {
        Callback::from(move |_| {
            log::info!(
                "Tile with coordinate {:?} has been clicked.",
                index.to_tuple()
            );
            board.dispatch(BoardAction::TurnCell(index));
        })
    }

    fn dispatch_change_cell(board: UseReducerHandle<BoardState>, index: Coordinate<isize>) -> Callback<WheelEvent> {
        Callback::from(move |_| {
            log::info!(
                "Tile with coordinate {:?} has been wheeled.",
                index.to_tuple()
            );
            board.dispatch(BoardAction::ChangeTileShape(index));
        })
    }

    let board = props.board.clone();
    let level_grid = board.level_grid.clone();
    let (height, width) = level_grid.dimensions().to_tuple();
    let (height, width) = (height as isize, width as isize);

    html!{
            <GridComponent> 
            {
                (0..height).into_iter().map(| row | {
                    html!{
                        <RowComponent key={row}>
                            {
                                (0..width).into_iter().map(| column | {
                                    let index = Coordinate { row, column };
                                    let tile = level_grid.get(index).unwrap().clone();
                                    html!{
                                        <CellComponent
                                            key={column}
                                            tile={tile}
                                            row_number={row}
                                            column_number={column}
                                            on_click={ if props.can_turn { dispatch_turn_cell(board.clone(), index) } else { Callback::from(|_|{}) } }
                                            on_wheel={ if props.can_change { dispatch_change_cell(board.clone(), index) } else { Callback::from(|_|{}) } }
                                        ></CellComponent>
                                    }
                                }).collect::<Html>()
                            }
                        </RowComponent>
                    }
                }).collect::<Html>()
            }
        </GridComponent>
    }
}


#[derive(Properties, PartialEq, Clone)]
pub struct StatelessLevelProps {
    pub level_grid: Grid<Tile<Square>>,
}

#[function_component(StatelessLevelComponent)]
pub fn stateless_level_component(props: &StatelessLevelProps) -> html {
    let level_grid = props.level_grid.clone();
    let (height, width) = level_grid.dimensions().to_tuple();
    let (height, width) = (height as isize, width as isize);

    html!{
            <GridComponent> 
            {
                (0..height).into_iter().map(| row | {
                    html!{
                        <RowComponent key={row}>
                            {
                                (0..width).into_iter().map(| column | {
                                    let index = Coordinate { row, column };
                                    let tile = level_grid.get(index).unwrap().clone();
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
    }
}