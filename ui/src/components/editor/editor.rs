use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;
use yew::prelude::*;
use yew::{html, Callback};
use game::model::gameboard::GameBoard;

use crate::components::editor::editor_reducer::{EditorAction, EditorState};

use crate::components::editor::grid::GridComponent;

use crate::helper::screen::Screen;

#[derive(Properties, PartialEq, Clone)]
pub struct EditorComponentProps {
    pub screen: UseStateHandle<Screen>,
}

#[function_component(EditorComponent)]
pub fn editor_component(props: &EditorComponentProps) -> Html {
    let new_grid = generate(Coordinate { row: 5, column: 5 }, 99);
    let editor = use_reducer_eq(EditorState::set(new_grid));

    let map_grid = editor.grid.clone();

    let generate_fast_gen_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Generate FastGen.");
            editor.dispatch(EditorAction::GenerateFastGen);
        })
    };

    let generate_wfc_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Generate WFC.");
            editor.dispatch(EditorAction::GenerateWFC);
        })
    };

    let check_cps_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Check with CPS.");
            log::info!("Current grid\n{}", map_grid.to_string());

            let solution_num = map_grid.solve().count();
            log::info!("Is valid grid? {}",
            match solution_num {
                0 => "No".to_string(),
                n => format!("Yes, and it has {} possible solutions", n),
            });

            editor.dispatch(EditorAction::ShowMessage(match solution_num {
                0 => String::from("The level is not valid"),
                n => format!("The level is valid and has {} possible solutions", n),
            }));
        })
    };

    // let check_sat_onclick: Callback<MouseEvent> = {
    //     Callback::from(move |_| {
    //         log::info!("[Button click] Check with SAT.");
    //         log::info!("Not implemented yet");
    //     })
    // };

    let check_solved_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Check is solved.");
            log::info!("Current grid\n{}", editor.grid.to_string());

            let is_solved = editor.grid.is_solved();
            log::info!("Is solved? {}", is_solved);

            editor.dispatch(EditorAction::ShowMessage(match is_solved {
                true => String::from("The level is solved"),
                false => String::from("The level is not solved")
            }));
        })
    };

    let shuffle_tile_rotations_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Shuffle tile rotations.");
            editor.dispatch(EditorAction::ShuffleTileRotations);
        })
    };

    let play_onclick: Callback<MouseEvent> = {
        let s = props.screen.clone();
        let g = editor.grid.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Play custom grid.");
            log::info!("Current grid\n{}", g.to_string());
            s.set(Screen::Level(g.clone()))
        })
    };

    let resize_width_plus_one_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize width +1.");
            log::info!(
                "[Button click] Resize width +1.{} {}",
                editor.grid_size.column + 1,
                editor.grid_size.row
            );
            editor.dispatch(EditorAction::ChangeSize(Coordinate {
                column: editor.grid_size.column + 1,
                row: editor.grid_size.row,
            }));
        })
    };

    let resize_width_minus_one_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize width -1.");
            if editor.grid_size.column > 1 {
                log::info!(
                    "[Button click] Resize width +1.{} {}",
                    editor.grid_size.column - 1,
                    editor.grid_size.row
                );
                editor.dispatch(EditorAction::ChangeSize(Coordinate {
                    column: editor.grid_size.column - 1,
                    row: editor.grid_size.row,
                }));
            }
        })
    };

    let resize_height_plus_one_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize height +1.");
            log::info!(
                "[Button click] Resize width +1.{} {}",
                editor.grid_size.column,
                editor.grid_size.row + 1
            );
            editor.dispatch(EditorAction::ChangeSize(Coordinate {
                column: editor.grid_size.column,
                row: editor.grid_size.row + 1,
            }));
        })
    };

    let resize_height_minus_one_onclick: Callback<MouseEvent> = {
        let editor = editor.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Resize height -1.");
            if editor.grid_size.row > 1 {
                log::info!(
                    "[Button click] Resize width +1.{} {}",
                    editor.grid_size.column,
                    editor.grid_size.row - 1
                );
                editor.dispatch(EditorAction::ChangeSize(Coordinate {
                    column: editor.grid_size.column,
                    row: editor.grid_size.row - 1,
                }));
            }
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
        <>
            <section id="controller">
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
                </ul>
            </section>
            <GridComponent editor_state={editor.clone()} />
            <div id="controller">
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
                <p>{editor.message.clone()}</p>
            </div>
        </>
    }
}
