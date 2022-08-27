use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
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
    let head_message = use_state_eq(|| "".to_string());

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
        let head_message = head_message.clone();
        Callback::from(move |_| {
            head_message.set("There is no way out of an infinite loop!".to_string());
        })
    };

    {
        let head_message = head_message.clone();
        use_effect_with_deps(
            move |message| {
                let message_string = (*message.clone()).clone();
                if message_string != "".to_string() {
                    let window = web_sys::window().unwrap();
                    let document = window.document().unwrap();
                    let msg_element = document.get_element_by_id("head-message").unwrap();
                    msg_element.remove_attribute("hidden").ok();
                    log::info!("-> current message is: {}", message_string);
                    
                    let hide_action = 
                    {
                        let message = message.clone();
                        let msg_element = msg_element.clone();
                        Closure::<dyn Fn()>::new(move || {
                            msg_element.set_attribute("hidden", "true").ok();
                            message.set("".to_string());   
                        })
                    };
                
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(hide_action.as_ref().unchecked_ref(), 3000)
                        .ok();
    
                    hide_action.forget();
                    
                }

            || ()
        }, 
        head_message
        )
    }

    html! {
        <>
            <div id="head-message" hidden=true>
                {(*head_message).clone()}
            </div>
            <div id="container">
                {
                    match &*screen {
                        Screen::Title => {
                            html! {
                                <div id="start-menu">
                                    <button onclick={to_preview}>
                                        {"-play-"}
                                    </button>
                                    <button onclick={to_editor}>
                                    {"-editor-"}
                                    </button>
                                    <button onclick={to_visualizer}>
                                    {"-viz-"}
                                    </button>
                                    <button onclick={to_help}>
                                        {"-help-"}
                                    </button>
                                    <button onclick={to_credit}>
                                        {"-credit-"}
                                    </button>
                                    <button onclick={on_exit}>
                                        {"-exit-"}
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
                                    <EditorComponent screen={screen.clone()} message={head_message.clone()}/>
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
        </>
    }
}
