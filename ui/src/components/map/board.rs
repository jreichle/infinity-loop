use crate::helper::screen::Screen;
use yew::prelude::*;
use yew::{html, Callback};

use game::model::gameboard::GameBoard;
use game::model::{
    grid::Grid,
    tile::{Square, Tile},
};

use crate::components::map::{
    board_reducer::{BoardAction, BoardState},
    grid::GridComponent,
};

use crate::helper::local_storage::change_screen;

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
            change_screen(screen.clone(), Screen::Overview);
        })
    };

    html! {
        <>
            <GridComponent board_state={board} />
            <div id="controller">
                <button
                    onclick={check_onclick}>
                    {"-check-"}
                    // {"🔍"}
                    // <img src="icons/magnifying-glass.svg" alt="Check if solved" />
                </button>
                <button
                    onclick={hint_onclick}>
                    {"-hint-"}
                    // {"💡"}
                    // <div class="light-bulb"></div>
                    // <img src="icons/light-bulb.svg" alt="Get hint" />
                </button>
                <button
                    onclick={solve_onclick}>
                    {"-solve-"}
                    // {"🔮"}
                    // <img src="icons/magic.svg" alt="Solve level" />
                </button>
                <button
                    onclick={next_onclick}>
                    {"-next-"}
                    // {"⏭️"}
                    // <img src="icons/next.svg" alt="Next level" />
                </button>
                <button
                    onclick={to_preview}>
                    {"-back-"}
                </button>
            </div>
        </>
    }
}
