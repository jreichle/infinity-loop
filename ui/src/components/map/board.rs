use game::model::tile::Square;
use yew::prelude::*;
use yew::{html, Html, Callback};

use game::model::{
    grid::Grid,
    tile::Tile,
};

use crate::components::map::
    {board_reducer::{BoardAction,BoardState},
    grid::GridComponent
};

#[derive(Properties, PartialEq, Clone)]
pub struct BoardComponentProps {
    pub level_grid: Grid<Tile<Square>>,
}

#[function_component(BoardComponent)]
pub fn board_component(props: &BoardComponentProps) -> Html {    
    let board = use_reducer_eq(BoardState::set(props.level_grid.clone(), true));
    let level_grid = board.level_grid.clone();

    let check_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            log::info!("LEVEL\n{}", level_grid.to_string());
            log::info!("[Button click] Check.");
        })
    };

    let hint_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            board.dispatch(BoardAction::GetHint);
            log::info!("[Button click] Hint.");
        })
    };

    let solve_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            log::info!("[Button click] Solve.");
        })
    };

    let next_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            board.dispatch(BoardAction::NextLevel);
            log::info!("[Button click] Next.");
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
            </div>
        </>
    }
}