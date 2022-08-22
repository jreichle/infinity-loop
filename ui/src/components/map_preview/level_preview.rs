use yew::prelude::*;
use yew::{html, Html};

use super::level::LevelComponent;
use game::model::coordinate::Coordinate;
use game::model::fastgen::generate;

#[derive(Properties, PartialEq, Clone)]
pub struct LevelPreviewComponentProps {
    pub level_count: usize,
}

#[function_component(LevelPreviewComponent)]
pub fn level_preview_component(props: &LevelPreviewComponentProps) -> Html { 
    html!{
        <div id="preview-container">
            {
                (1..=props.level_count).into_iter().map( | level_index | {
                    html!{
                        <LevelComponent 
                            level_grid={generate(Coordinate { row: 5, column: 5 }, level_index.try_into().unwrap())} 
                            level_index={level_index} 
                            is_completed={ if level_index > 20 { false } else { true } }
                        />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}