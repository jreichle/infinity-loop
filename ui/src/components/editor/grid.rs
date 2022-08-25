use yew::prelude::*;
use yew::{html, Html};

use crate::components::editor::row::RowComponent;

use super::editor_reducer::EditorState;

#[derive(Properties, PartialEq, Clone)]
pub struct GridComponentProps {
    pub editor_state: UseReducerHandle<EditorState>,
}

#[function_component(GridComponent)]
pub fn grid_component(props: &GridComponentProps) -> Html {
    let editor = props.editor_state.clone();

    // -- FROM grid -> is_solved
    // let row_slice = |r| {
    //     (0..width)
    //         .map(|c| Coordinate { row: r, column: c })
    //         .collect::<Vec<_>>()
    // };

    // let column_slice = |c| {
    //     (0..height)
    //         .map(|r| Coordinate { row: r, column: c })
    //         .collect::<Vec<_>>()
    // };

    // TODO: only pass row state & cell state instead of the whole game state

    let level_grid = editor.grid.clone();
    let (height, _) = level_grid.dimensions().to_tuple();

    html! {
        <>
            <div class="game-board">
                {
                    (0..height).into_iter().map(| row_number | {
                        html!{
                            <RowComponent key={row_number} row_number={row_number as isize} editor_state={editor.clone()} />
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}
