use crate::helper::screen::Screen;
use yew::prelude::*;
use yew::{events::Event, html, Callback};

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

pub struct Comp;

use game::model::gameboard::GameBoard;
use game::model::{
    grid::Grid,
    tile::{Square, Tile},
};

use game::model::cnf;

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
    pub cnf: UseStateHandle<String>,
    pub literals: UseStateHandle<String>,
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

    let solve_onclick_input: Callback<MouseEvent> = {
        let board = board.clone();
        let literals = props.literals.clone();
        //let message = props.message.clone();
        //let level_grid = board.level_grid.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Solve.");
            board.dispatch(BoardAction::SolveLevelInput(literals.to_string()));
            // let msg = match level_grid.is_solved() {
            //     true => String::from("The level is solved"),
            //     false => String::from("This did not solve the level"),
            // };
            //message.set(msg);
        })
    };

    let generate_cnf: Callback<MouseEvent> = {
        let board = board.clone();
        let cnf = props.cnf.clone();
        let level_grid = board.data.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Generate cnf.");
            cnf.set(cnf::level_to_cnf(&level_grid.clone()).unwrap());
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

    let on_input: Callback<Event> = {
        log::info!("[Button click] onInput");
        let literals = props.literals.clone();
        Callback::from(move |e: Event| {
            let target = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            log::info!("[Button click] onInput {}", input.clone().unwrap().value());
            literals.set(input.unwrap().value());
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
                    onclick={generate_cnf}>
                    {"-generate cnf-"}
                </button>

                <input
                    onchange={on_input}
                    id="my-input"
                    type="text"
                    placeholder="Put the literals in DIMACS format here"
                />

                <button
                    onclick={solve_onclick_input}>
                    {"-solve with DIMACS input-"}
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
