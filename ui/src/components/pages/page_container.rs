use yew::html;
use yew::prelude::*;

use crate::components::editor::editor::EditorComponent;
use crate::components::map::board::BoardComponent;
use crate::components::map_preview::level_preview::LevelPreviewComponent;
use crate::components::wfc_visualizer::wfc_board::WfcBoardComponent;
use crate::helper::screen::Screen;

use game::model::coordinate::Coordinate;

#[function_component(PageContainer)]
pub fn page_container() -> Html {
    let dimension = use_state(|| Coordinate::new(5 as usize, 5 as usize));
    let level_number = use_state(|| 0);
    let screen = use_state(|| Screen::Title);

    let to_preview: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.clone().set(Screen::Overview);
        })
    };

    let to_editor: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.clone().set(Screen::Editor);
        })
    };

    let to_visualizer: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.clone().set(Screen::Visualizer);
        })
    };

    html! {
        <div id="container">
            {
                match &*screen {
                    Screen::Title => {
                        html! {
                            <>
                                <button onclick={to_preview}>
                                    {"-Preview Levels-"}
                                </button>
                                <button onclick={to_editor}>
                                    {"-Level Editor-"}
                                </button>
                                <button onclick={to_visualizer}>
                                    {"-Generation Visualizer-"}
                                </button>
                            </>
                        }
                    },
                    Screen::Overview => {
                        html! {
                            <LevelPreviewComponent
                                screen={screen.clone()}
                                dimension={dimension}
                                level_number={level_number}
                            />
                        }
                    },
                    Screen::Editor => {
                        html! {
                                <EditorComponent screen={screen.clone()}/>
                        }
                    },
                    Screen::Level(user_grid) => {
                        html! {
                            <BoardComponent
                                level_grid={user_grid.clone()}
                                screen={screen.clone()}/>

                        }
                    },
                    Screen::Visualizer => {
                        html!{
                            // <WfcBoardComponent screen={screen.clone()} />
                            <WfcBoardComponent />
                        }
                    }
                }
            }
        </div>
    }
}
