use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;
use yew::prelude::*;
use yew::{html, Callback, Html, InputEvent};

use game::model::gameboard::GameBoard;

use crate::components::map::board_reducer::{BoardState, BoardAction};
use crate::components::editor::editor_reducer::{EditorState, EditorAction};

use crate::components::map::grid::GridComponent;

use crate::helper::screen::Screen;

#[derive(Properties, PartialEq, Clone)]
pub struct EditorComponentProps {
    pub screen: UseStateHandle<Screen>,
}

#[function_component(EditorComponent)]
pub fn editor_component(props: &EditorComponentProps) -> Html {
    let new_grid = generate(Coordinate { row: 5, column: 5 }, 99);
    let editor = use_reducer_eq(EditorState::set(BoardState::new(new_grid, true)));

    let map_grid = editor.board.level_grid.clone();
    let (_, height) = map_grid.dimensions().to_tuple();

    let generateFastGen_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Generate FastGen.");
            editor.dispatch(EditorAction::GenerateFastGen);
        })
    };

    let generateWFC_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Generate WFC.");
            editor.dispatch(EditorAction::GenerateWFC);
        })
    };

    let checkCPS_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            log::info!("[Button click] Check with CPS.");
            log::info!("Current grid\n{}", map_grid.to_string());
            log::info!("Is valid grid? {}", match map_grid.solve().count() {
                0 => "No".to_string(),
                n => format!("Yes, and it has {} possible solutions", n),
            });
        })
    };

    let checkSAT_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            log::info!("[Button click] Check with SAT.");
            log::info!("Not implemented yet");
        })
    };

    let checkSolved_onclick: Callback<MouseEvent> = {
        let g = editor.board.level_grid.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Check is solved.");
            log::info!("Current grid\n{}", g.to_string());
            log::info!("Is solved? {}", g.is_solved());
        })
    };

    let play_onclick: Callback<MouseEvent> = {
        let s = props.screen.clone();
        let g = editor.board.level_grid.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Play custom grid.");
            log::info!("Current grid\n{}", g.to_string());
            s.set(Screen::Level(g.clone()))
        })
    };

    // TODO: only pass row state & cell state instead of the whole game state

    html! {
        <>
            <GridComponent board_state={use_reducer_eq(BoardState::set(editor.board.level_grid.clone(), true))} />
            <div id="controller">
                <button
                    onclick={generateFastGen_onclick}
                    >{"Generate with FastGen"}</button>
                <button
                    onclick={generateWFC_onclick}
                    >{"Generate with WFC"}</button>
                <button
                    onclick={checkCPS_onclick}
                    >{"Check validity with Constraint Propagation Solver"}</button>
                <button
                    onclick={checkSAT_onclick}
                    >{"Check validity with SAT Solver"}</button>
                <button
                    onclick={checkSolved_onclick}
                    >{"Check if solved"}</button>
                <button
                    onclick={play_onclick}
                    >{"Play"}</button>
            </div>
        </>
    }
}
