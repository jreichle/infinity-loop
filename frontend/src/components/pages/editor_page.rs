use yew::prelude::*;
use yew::{html, Callback};

use game::generator::fastgen::generate;
use game::model::{
    gameboard::GameBoard,
    coordinate::Coordinate
};

use crate::components::board::level::LevelComponent;
use crate::components::reducers::board_reducer::{BoardAction, Level};
use crate::helper::local_storage::{change_screen, save_editor_level};
use crate::helper::screen::Screen;

#[derive(Properties, PartialEq, Clone)]
pub struct EditorPageProps {
    pub screen: UseStateHandle<Screen>,
    pub head_message: UseStateHandle<String>,
}

#[function_component(EditorPage)]
pub fn editor_page_component(props: &EditorPageProps) -> Html {
    let new_grid = generate(Coordinate { row: 5, column: 5 }, 99);
    let board = use_reducer_eq(Level::set_grid(new_grid));

    let level_grid = board.data.clone();

    let clear_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Generate FastGen.");
            board.dispatch(BoardAction::ClearGrid);
        })
    };

    let generate_fast_gen_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Generate FastGen.");
            board.dispatch(BoardAction::GenerateFastGen);
        })
    };

    let generate_wfc_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Generate WFC.");
            board.dispatch(BoardAction::GenerateWFC);
        })
    };

    let check_cps_onclick: Callback<MouseEvent> = {
        // let board = board.clone();
        let head_message = props.head_message.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Check with CPS.");
            log::info!("Current grid\n{}", level_grid.to_string());

            let solution_num = level_grid.solve().count();
            log::info!(
                "Is valid grid? {}",
                match solution_num {
                    0 => "No".to_string(),
                    n => format!("Yes, and it has {} possible solutions", n),
                }
            );

            let msg = match solution_num {
                0 => String::from("The level is not valid"),
                n => format!("The level is valid and has {} possible solutions", n),
            };
            head_message.set(msg);
        })
    };

    let check_solved_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        let head_message = props.head_message.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Check is solved.");
            log::info!("Current grid\n{}", board.data.to_string());

            let is_solved = board.data.is_solved();
            log::info!("Is solved? {}", is_solved);

            let msg = match is_solved {
                true => String::from("The level is solved"),
                false => String::from("The level is not solved"),
            };
            head_message.set(msg);
        })
    };

    let shuffle_tile_rotations_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Shuffle tile rotations.");
            board.dispatch(BoardAction::ShuffleTileRotations);
        })
    };

    let play_onclick: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        let grid = board.data.clone();
        let head_message = props.head_message.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Play custom grid.");
            log::info!("Current grid\n{}", grid.to_string());
            if grid.solve().count() != 0 {
                change_screen(screen.clone(), Screen::Level(grid.clone()));
            } else {
                head_message.set(String::from(
                    "The level is not valid and thus not playable.",
                ));
            }
        })
    };

    let resize_width_plus_one_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize width +1.");
            log::info!(
                "[Button click] Resize width +1.{} {}",
                board.data.dimensions().column + 1,
                board.data.dimensions().row
            );
            board.dispatch(BoardAction::ChangeSize(Coordinate {
                column: board.data.dimensions().column + 1,
                row: board.data.dimensions().row,
            }));
        })
    };

    let resize_width_minus_one_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize width -1.");
            if board.data.dimensions().column > 1 {
                log::info!(
                    "[Button click] Resize width +1.{} {}",
                    board.data.dimensions().column - 1,
                    board.data.dimensions().row
                );
                board.dispatch(BoardAction::ChangeSize(Coordinate {
                    column: board.data.dimensions().column - 1,
                    row: board.data.dimensions().row,
                }));
            }
        })
    };

    let resize_height_plus_one_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize height +1.");
            log::info!(
                "[Button click] Resize width +1.{} {}",
                board.data.dimensions().column,
                board.data.dimensions().row + 1
            );
            board.dispatch(BoardAction::ChangeSize(Coordinate {
                column: board.data.dimensions().column,
                row: board.data.dimensions().row + 1,
            }));
        })
    };

    let resize_height_minus_one_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize height -1.");
            if board.data.dimensions().row > 1 {
                log::info!(
                    "[Button click] Resize width +1.{} {}",
                    board.data.dimensions().column,
                    board.data.dimensions().row - 1
                );
                board.dispatch(BoardAction::ChangeSize(Coordinate {
                    column: board.data.dimensions().column,
                    row: board.data.dimensions().row - 1,
                }));
            }
        })
    };

    let save_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        let head_message = props.head_message.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Save level.");
            save_editor_level(&board.data);
            head_message.set(String::from("Saved level"));
        })
    };

    let preview_onclick: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::info!("To Preview");
            change_screen(screen.clone(), Screen::Overview)
        })
    };

    let to_title: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Editor");
            change_screen(screen.clone(), Screen::Title);
        })
    };

    html! {
        <div class="container editor-page">
            <section class="controller">
                <ul style="list-style-type: none">
                    <li><button
                        onclick={resize_height_minus_one_onclick}
                        style="width:50px;height:50px;margin-left:55px;margin-right:20px"
                        >{"-"}</button></li>
                    <li><button
                        onclick={resize_width_minus_one_onclick}
                        style="width:50px;height:50px"
                        >{"-"}</button>
                    <b style="width:50px;height:50px">{"Resize"}</b>
                    <button
                        onclick={resize_width_plus_one_onclick}
                        style="width:50px;height:50px;margin-right:20px"
                        >{"+"}</button></li>
                    <li><button
                        onclick={resize_height_plus_one_onclick}
                        style="width:50px;height:50px;margin-left:55px;margin-right:20px"
                        >{"+"}</button></li>
                </ul>
            </section>

            <LevelComponent
                board={board.clone()}
                can_complete=false
                can_turn=true
                can_change=true
                head_message={props.head_message.clone()} />

            <div class="controller">
                <button
                    onclick={generate_fast_gen_onclick}
                    >{"-Generate with FastGen-"}</button>
                <button
                    onclick={generate_wfc_onclick}
                    >{"-Generate with WFC-"}</button>
                <p style="text-align:center;margin:20px">{"____"}</p>
                <button
                    onclick={check_cps_onclick}
                    >{"-Check validity-"}</button>
                <button
                    onclick={check_solved_onclick}
                    >{"-Check if solved-"}</button>
                <p style="text-align:center;margin:20px">{"____"}</p>
                <button
                    onclick={clear_onclick}
                    >{"-Clear grid-"}</button>
                <button
                    onclick={shuffle_tile_rotations_onclick}
                    >{"-Shuffle tile rotations-"}</button>
                <p style="text-align:center;margin:20px">{"____"}</p>
                <button
                    onclick={save_onclick}
                    >{"-Save-"}</button>
                <button
                    onclick={play_onclick}
                    >{"-Play-"}</button>
                <button  onclick={preview_onclick}>
                    {"-Levels-"}
                </button>
                <button  onclick={to_title}>
                    {"-home-"}
                </button>
            </div>
        </div>
    }
}
