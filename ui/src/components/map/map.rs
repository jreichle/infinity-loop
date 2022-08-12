use yew::prelude::*;
use yew::{html, Html, Callback};

use super::map_reducer::MapState;
use super::row::RowComponent;

#[function_component(MapComponent)]
pub fn map_component() -> Html {    
// pub fn map_component(props: &MapComponentProps) -> Html {
    // let level = parse_level(props.level_data.clone());

    let map = use_reducer_eq(MapState::default);
    
    // let map_context = use_state(|| props.map_layout.clone());

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
            log::info!("[Button click] Solve.")
        })
    };

    let next_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            log::info!("[Button click] Next.")
        })
    };

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