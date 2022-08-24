use yew::prelude::*;
use yew::{html, Html};

use super::preview_reducer::PreviewState;
use crate::components::map::cell::{get_angle, get_index};
use crate::helper::screen::Screen;

use rand::Rng;

use game::model::gameboard::GameBoard;
use game::model::{
    coordinate::Coordinate,
    grid::Grid,
    tile::{Square, Tile},
};

#[derive(Properties, PartialEq, Clone)]
pub struct LevelComponentProps {
    pub preview_state: UseReducerHandle<PreviewState>,
    pub screen: UseStateHandle<Screen>,
    pub level_index: usize,
}

#[function_component(LevelComponent)]
pub fn level_preview_component(props: &LevelComponentProps) -> Html {
    let level_grid = props.preview_state.extracted_levels[props.level_index].clone();
    let (height, width) = level_grid.dimensions().to_tuple();

    let to_level: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        let level = level_grid.clone();
        Callback::from(move |_| {
            screen.set(Screen::Level(level.clone()));
        })
    };

    let img_path = vec![
        "data/tiles/0.svg",
        "data/tiles/1.svg",
        "data/tiles/2.svg",
        "data/tiles/3.svg",
        "data/tiles/4.svg",
        "data/tiles/5.svg",
    ];

    html! {
        <div class="level-container" onclick={to_level}>
                {
                    (0..height).into_iter().map(| row | {
                        html!{
                            <div class="cell-row">
                            {
                                (0..width).into_iter().map(| column | {
                                    let cell_symbol = level_grid
                                        .get(Coordinate {
                                                row: row.try_into().unwrap(),
                                                column: column.try_into().unwrap()
                                            })
                                        .unwrap().to_string()
                                        .chars().next().unwrap();
                                    html!{
                                        <div class="preview-cell">
                                            <img
                                                src={img_path[get_index(cell_symbol)]}
                                                style={format!("{}{}{}",
                                                    "transform:rotate(",
                                                    get_angle(cell_symbol),
                                                    "deg);")}
                                            />
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                            </div>
                        }
                    }).collect::<Html>()
                }
            <div class="level-title">{format!("#{}", props.level_index + 1)}</div>
        </div>
    }
}
