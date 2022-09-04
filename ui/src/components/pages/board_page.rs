use crate::helper::screen::Screen;
use yew::prelude::*;
use yew::{html, Callback};

use game::model::gameboard::GameBoard;
use game::model::{
    grid::Grid,
    tile::{Square, Tile},
};

use game::model::knf;

use crate::components::board::level::LevelComponent;
use crate::components::reducers::board_reducer::{BoardAction, BoardState};

use crate::helper::local_storage::change_screen;

#[derive(Properties, PartialEq, Clone)]
pub struct BoardPageProps {
    pub level_grid: Grid<Tile<Square>>,
    pub screen: UseStateHandle<Screen>,
    pub message: UseStateHandle<String>,   
    pub cnf: UseStateHandle<String>,
    pub literals: UseStateHandle<String>,
}


#[function_component(BoardPage)]
pub fn board_page_component(props: &BoardPageProps) -> Html {
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

    let solve_onclick_input: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Solve.");
            board.dispatch(BoardAction::SolveLevelInput);
        })
    };

    let generate_cnf: Callback<MouseEvent> = {
        let board = board.clone();
        let cnf = props.cnf.clone();
        let level_grid = board.level_grid.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Generate cnf.");
            //props.cnf.set(board.dispatch(BoardAction::GenerateCnf));
            cnf.set(knf::level_to_cnf(&level_grid.clone()).unwrap());
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
        <div class="container">
            <LevelComponent board={board.clone()} can_turn=true can_change=false />
            <div class="controller">
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
                    onclick={generate_cnf}>
                    {"-generate cnf-"}
                </button>

                <textarea title="literals_input">
                    {{literals}}
                </textarea>
                
                <button
                    onclick={solve_onclick_input}>
                    {"-solve with input-"}
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
