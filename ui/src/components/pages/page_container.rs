use yew::html;
use yew::prelude::*;

use crate::components::editor::editor::EditorComponent;
use crate::components::map::board::BoardComponent;
use crate::components::map_preview::level_preview::LevelPreviewComponent;
use crate::components::pages::credit_page::CreditPage;
use crate::components::pages::help_page::HelpPage;
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

    let to_help: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.clone().set(Screen::Help);
        })
    };

    let to_credit: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            screen.clone().set(Screen::Credit);
        })
    };

    let on_exit: Callback<MouseEvent> = {
        Callback::from(move |_| {
            web_sys::window()
                .unwrap()
                .alert_with_message("There is no way out of an infinite loop!")
                .ok();
        })
    };

    html! {
        <div id="container">
            {
                match &*screen {
                    Screen::Title => {
                        html! {
                            <div id="start-menu">
                                <button onclick={to_preview}>
                                    {"-Play-"}
                                </button>
                                <button onclick={to_editor}>
                                    {"-Level Editor-"}
                                </button>
                                <button onclick={to_visualizer}>
                                    {"-Generation Visualizer-"}
                                </button>
                                <button onclick={to_help}>
                                    {"-Help-"}
                                </button>
                                <button onclick={to_credit}>
                                    {"-Credit-"}
                                </button>
                                <button onclick={on_exit}>
                                    {"-Exit-"}
                                </button>
                            </div>
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
                            <WfcBoardComponent screen={screen.clone()} />
                        }
                    },
                    Screen::Help => {
                        html!{
                           <HelpPage screen={screen.clone()} />
                        }
                    },
                    Screen::Credit => {
                        html!{
                            <CreditPage screen={screen.clone()} />
                        }
                    }
                }
            }
        </div>
    }
}
