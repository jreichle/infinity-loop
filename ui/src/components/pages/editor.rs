use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;
use game::model::gameboard::GameBoard;
use yew::prelude::*;
use yew::{html, Callback};

use crate::components::reducers::board_reducer::{BoardAction, BoardState};
use crate::components::map::level::LevelComponent;
use crate::helper::screen::Screen;

#[derive(Properties, PartialEq, Clone)]
pub struct EditorComponentProps {
    pub screen: UseStateHandle<Screen>,
    pub message: UseStateHandle<String>,
}

#[function_component(EditorComponent)]
pub fn editor_component(props: &EditorComponentProps) -> Html {
    let new_grid = generate(Coordinate { row: 5, column: 5 }, 99);
    let board = use_reducer_eq(BoardState::set_grid(new_grid));

    let level_grid = board.level_grid.clone();

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
        let message = props.message.clone();
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
            message.set(msg);
        })
    };

    // let check_sat_onclick: Callback<MouseEvent> = {
    //     Callback::from(move |_| {
    //         log::info!("[Button click] Check with SAT.");
    //         log::info!("Not implemented yet");
    //     })
    // };

    let check_solved_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        let message = props.message.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Check is solved.");
            log::info!("Current grid\n{}", board.level_grid.to_string());

            let is_solved = board.level_grid.is_solved();
            log::info!("Is solved? {}", is_solved);

            let msg = match is_solved {
                true => String::from("The level is solved"),
                false => String::from("The level is not solved"),
            };
            message.set(msg);
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
        let s = props.screen.clone();
        let g = board.level_grid.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Play custom grid.");
            log::info!("Current grid\n{}", g.to_string());
            s.set(Screen::Level(g.clone()))
        })
    };

    let resize_width_plus_one_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize width +1.");
            log::info!(
                "[Button click] Resize width +1.{} {}",
                board.level_grid.dimensions().column + 1,
                board.level_grid.dimensions().row
            );
            board.dispatch(BoardAction::ChangeSize(Coordinate {
                column: board.level_grid.dimensions().column + 1,
                row: board.level_grid.dimensions().row,
            }));
        })
    };

    let resize_width_minus_one_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize width -1.");
            if board.level_grid.dimensions().column > 1 {
                log::info!(
                    "[Button click] Resize width +1.{} {}",
                    board.level_grid.dimensions().column - 1,
                    board.level_grid.dimensions().row
                );
                board.dispatch(BoardAction::ChangeSize(Coordinate {
                    column: board.level_grid.dimensions().column - 1,
                    row: board.level_grid.dimensions().row,
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
                board.level_grid.dimensions().column,
                board.level_grid.dimensions().row + 1
            );
            board.dispatch(BoardAction::ChangeSize(Coordinate {
                column: board.level_grid.dimensions().column,
                row: board.level_grid.dimensions().row + 1,
            }));
        })
    };

    let resize_height_minus_one_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize height -1.");
            if board.level_grid.dimensions().row > 1 {
                log::info!(
                    "[Button click] Resize width +1.{} {}",
                    board.level_grid.dimensions().column,
                    board.level_grid.dimensions().row - 1
                );
                board.dispatch(BoardAction::ChangeSize(Coordinate {
                    column: board.level_grid.dimensions().column,
                    row: board.level_grid.dimensions().row - 1,
                }));
            }
        })
    };

    let save_onclick: Callback<MouseEvent> = {
        let board = board.clone();
        let message = props.message.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Save level.");

            let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
            let key = format!("Own level {}", (local_storage.length().unwrap() + 1));
            local_storage
                .set_item(key.as_str(), board.level_grid.to_string().as_str())
                .unwrap();

            let msg = format!("Level saved as \"{}\"", key);
            message.set(msg);
        })
    };

    let preview_onclick: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::info!("To Preview");
            screen.set(Screen::Overview);
        })
    };

    let back_onclick: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Editor");
            screen.set(Screen::Title);
        })
    };

    html! {
        <div class="container">
            <section class="controller">
                <ul style="list-style-type: none">
                    <li><button
                        onclick={resize_height_minus_one_onclick}
                        style="width:80px;height:50px;margin-left:65px;margin-right:20px"
                        >{"-"}</button></li>
                    <li><button
                        onclick={resize_width_minus_one_onclick}
                        style="width:80px;height:50px"
                        >{"-"}</button>
                    <b style="width:80px;height:50px">{"Resize"}</b>
                    <button
                        onclick={resize_width_plus_one_onclick}
                        style="width:80px;height:50px;margin-right:20px"
                        >{"+"}</button></li>
                    <li><button
                        onclick={resize_height_plus_one_onclick}
                        style="width:80px;height:50px;margin-left:65px;margin-right:20px"
                        >{"+"}</button></li>
                    <li>
                        <button
                            onclick={save_onclick}
                            style="margin-left:65px;margin-right:20px, margin-top=20px"
                            >{"-Save-"}</button>
                    </li>
                </ul>
            </section>

            <LevelComponent board={board.clone()} can_turn=true can_change=true />

            <div class="controller">
                <button
                    onclick={clear_onclick}
                    >{"-Clear grid-"}</button>
                <button
                    onclick={generate_fast_gen_onclick}
                    >{"-Generate with FastGen-"}</button>
                <button
                    onclick={generate_wfc_onclick}
                    >{"-Generate with WFC-"}</button>
                <button
                    onclick={check_cps_onclick}
                    >{"-Check validity-"}</button>
                // <button
                //     onclick={check_sat_onclick}
                //     >{"-Check validity with SAT Solver-"}</button>
                <button
                    onclick={check_solved_onclick}
                    >{"-Check if solved-"}</button>
                <button
                    onclick={shuffle_tile_rotations_onclick}
                    >{"-Shuffle tile rotations-"}</button>
                <button
                    onclick={play_onclick}
                    >{"-Play-"}</button>
                <button  onclick={preview_onclick}>
                    {"-Levels-"}
                </button>
                <button  onclick={back_onclick}>
                    {"-back-"}
                </button>
            </div>
        </div>
    }
}
