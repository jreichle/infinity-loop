use crate::helper::screen::Screen;
use yew::prelude::*;
use yew::{html, Callback};

use game::model::{
    grid::Grid,
    tile::{Square, Tile},
};

use crate::components::map::{
    board_reducer::{BoardAction, BoardState},
    grid::GridComponent,
};

#[derive(Properties, PartialEq, Clone)]
pub struct BoardComponentProps {
    pub level_grid: Grid<Tile<Square>>,
    pub screen: UseStateHandle<Screen>,
}

#[function_component(BoardComponent)]
pub fn board_component(props: &BoardComponentProps) -> Html {
    let board = use_reducer_eq(BoardState::set_grid(props.level_grid.clone()));

    let check_onclick: Callback<MouseEvent> = {
        let level_grid = board.level_grid.clone();
        Callback::from(move |_| {
            log::info!("LEVEL\n{}", level_grid.to_string());
            log::info!("[Button click] Check.");
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

    let editor_onclick: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Editor");
            screen.set(Screen::Editor)
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
                    onclick={editor_onclick}
                    style="margin-top:20px">
                    {"-editor-"}
                </button>
            </div>
        </>
    }
}
