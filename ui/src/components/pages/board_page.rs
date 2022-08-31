use crate::helper::screen::Screen;
use yew::prelude::*;
use yew::{html, Callback};

use game::model::gameboard::GameBoard;
use game::model::{
    grid::Grid,
    tile::{Square, Tile},
};

use crate::components::board::level::LevelComponent;
use crate::components::reducers::board_reducer::{BoardAction, BoardState};

use crate::helper::local_storage::change_screen;

#[derive(Properties, PartialEq, Clone)]
pub struct BoardPageProps {
    pub level_grid: Grid<Tile<Square>>,
    pub screen: UseStateHandle<Screen>,
    pub message: UseStateHandle<String>,
}

#[function_component(BoardPage)]
pub fn board_page_component(props: &BoardPageProps) -> Html {
    let board = use_reducer_eq(BoardState::set_grid(props.level_grid.clone()));

    let hint_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        let message = props.message.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Hint.");
            if board.level_grid.solve().count() == 1 {
                board.dispatch(BoardAction::GetHint);
            } else {
                // hinting only works for one solution thus far!
                message.set(String::from(
                    "Hint can unfortunately not be generated for this level",
                ));
            }
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
        let message = props.message.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Next.");
            if board.level_grid.is_solved() {
                board.dispatch(BoardAction::NextLevel);
            } else {
                message.set(String::from("Solve the level to unlock a new level."));
            }
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
        <div class="container">
            <LevelComponent
                board={board.clone()}
                can_turn=true
                can_change=false
                message={props.message.clone()}/>
            <div class="controller">
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
        </div>
    }
}
