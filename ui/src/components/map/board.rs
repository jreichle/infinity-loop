use crate::helper::screen::Screen;
use yew::prelude::*;
use yew::{html, Callback};

use game::model::gameboard::GameBoard;
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
pub struct BoardComponentProps {
    pub level_grid: Grid<Tile<Square>>,
    pub screen: UseStateHandle<Screen>,
    pub message: UseStateHandle<String>,
}

#[function_component(BoardComponent)]
pub fn board_component(props: &BoardComponentProps) -> Html {
    let board = use_reducer_eq(BoardState::set_grid(props.level_grid.clone()));

    let check_onclick: Callback<MouseEvent> = {
        let level_grid = board.level_grid.clone();
        let message = props.message.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Check.");
            let msg = match level_grid.is_solved() {
                true => String::from("The level is solved"),
                false => String::from("The level is not solved"),
            };
            message.set(msg);
        })
    };

    let hint_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Hint.");
            board.dispatch(BoardAction::GetHint);
        })
    };

    let solve_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Solve.");
            board.dispatch(BoardAction::SolveLevel);
        })
    };

    let next_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Next.");
            board.dispatch(BoardAction::NextLevel);
        })
    };

    let to_preview: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Editor");
            screen.set(Screen::Overview);
        })
    };


    fn get_turn_callback(board: UseReducerHandle<BoardState>, index: Coordinate<isize>) -> Callback<MouseEvent> {
        Callback::from(move |_| {
            log::info!(
                "Tile with coordinate ({:?}) has been clicked.",
                index.to_tuple()
            );
            board.dispatch(BoardAction::TurnCell(index));
        })
    }

    let (height, width) = board.level_grid.dimensions().to_tuple();
    let (height, width) = (height as isize, width as isize);

    html! {
        <>
            <GridComponent> 
                {
                    (0..height).into_iter().map(| row | {
                        html!{
                            <RowComponent key={row}>
                                {
                                    (0..width).into_iter().map(| column | {
                                        let index = Coordinate { row, column };
                                        let tile = board.level_grid.get(index).unwrap().clone();
                                        html!{
                                            <CellComponent
                                                key={column}
                                                tile={tile}
                                                row_number={row}
                                                column_number={column}
                                                on_click={get_turn_callback(board.clone(), index)}
                                            ></CellComponent>
                                        }
                                    }).collect::<Html>()
                                }
                            </RowComponent>
                        }
                    }).collect::<Html>()
                }
            </GridComponent>

            <div id="controller">
                <button
                    onclick={check_onclick}>
                    {"-check-"}
                </button>
                <button
                    onclick={hint_onclick}>
                    {"-hint-"}

                </button>
                <button
                    onclick={solve_onclick}>
                    {"-solve-"}
                </button>
                <button
                    onclick={next_onclick}>
                    {"-next-"}
                </button>
                <button
                    onclick={to_preview}>
                    {"-back-"}
                </button>
            </div>
        </>
    }
}
