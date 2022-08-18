use game::model::tile::Square;
use yew::prelude::*;
use yew::{html, Html, Callback};

use game::model::{
    grid::Grid,
    tile::Tile,
};

use crate::components::map::map_reducer::MapAction;

use super::map_reducer::MapState;
use super::row::RowComponent;

#[derive(Properties, PartialEq, Clone)]
pub struct RowComponentProps {
    pub grid_map: Grid<Tile<Square>>,
}

#[function_component(MapComponent)]
pub fn map_component(props: &RowComponentProps) -> Html {    
    let map = use_reducer_eq(MapState::set(props.grid_map.clone()));

    let map_grid = map.level_grid.clone();
    let (height, _) = map.level_size.to_tuple();

    let check_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            log::info!("LEVEL\n{}", map_grid.to_string());
            log::info!("[Button click] Check.");
        })
    };

    let solve_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            log::info!("[Button click] Solve.");
        })
    };

    let next_onclick: Callback<MouseEvent> = {
        let map = map.clone();
        Callback::from(move |_| {
            map.dispatch(MapAction::NextLevel);
            log::info!("[Button click] Next.");
        })
    };

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

    html! {
        <>
            <div class="game-board">
                {
                    (0..height).into_iter().map(| row_number | {
                        html!{ 
                            <RowComponent key={row_number} row_number={row_number as isize} map_state={map.clone()} /> 
                        }
                    }).collect::<Html>()
                }
            </div>
            <div id="controller">
                <button 
                    onclick={check_onclick} 
                    >{"check"}</button>
                <button
                    onclick={solve_onclick}
                    >{"solve"}</button>
                <button 
                    onclick={next_onclick}
                    >{"next"}</button>
            </div>
        </>
    }
}