use crate::helper::screen::Screen;
use yew::prelude::*;
use yew::{html, Callback};

use game::model::gameboard::GameBoard;
use game::model::{
    grid::Grid,
    tile::{Square, Tile},
};

use crate::components::board::level::LevelComponent;
use crate::components::reducers::board_reducer::{BoardAction, Level};

use crate::helper::local_storage::change_screen;

/// the props are used to initialize the board page
///
/// level_grid: level that is being played
/// screen: used to change screens away from board or to another level
/// head_message: can be used to show information to user
#[derive(Properties, PartialEq, Clone)]
pub struct BoardPageProps {
    pub level_grid: Grid<Tile<Square>>,
    pub screen: UseStateHandle<Screen>,
    pub head_message: UseStateHandle<String>,
}

/// this page can be used to play levels
///
/// functinality
/// - click and turn tiles
/// - hinting: shortly highlight a tile to help solve level
/// - solving the level
/// - load the next level
#[function_component(BoardPage)]
pub fn board_page_component(props: &BoardPageProps) -> Html {
    let board = use_reducer_eq(Level::set_grid(props.level_grid.clone()));

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
        let head_message = props.head_message.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Next.");
            if board.data.is_solved() {
                board.dispatch(BoardAction::NextLevel);
            } else {
                head_message.set(String::from("Solve the level to unlock a new level."));
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
                head_message={props.head_message.clone()}/>
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
                    {"-levels-"}
                </button>
            </div>
        </div>
    }
}
