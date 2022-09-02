use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;


use crate::components::pages::board_page::BoardPage;
use crate::components::pages::editor_page::EditorPage;
use crate::components::pages::start_page::StartPage;
use crate::components::pages::text_page::TextPage;
use crate::components::pages::visualizer_page::VisualizerPage;
use crate::components::pages::level_preview::LevelPreviewPage;

use crate::helper::local_storage::retrieve_screen;
use crate::helper::screen::Screen;

use game::model::coordinate::Coordinate;

#[function_component(PageRouter)]
pub fn page_router() -> Html {
    let head_message = use_state_eq(|| "".to_string());
    let head_message_timeout_id = use_state(|| -1_i32);

    let dimension = use_state(|| Coordinate::new(5 as usize, 5 as usize));
    let level_number = use_state(|| 0);

    let screen = use_state(|| retrieve_screen());

    // use effect to let head_message disappear after 1.5 seconds
    // depends on message change
    {
        let head_message = head_message.clone();
        let timeout_id = head_message_timeout_id.clone();
        use_effect_with_deps(
            move |head_message| {
                let window = web_sys::window().unwrap();
                if !head_message.trim().is_empty() {
                    if *timeout_id != -1 {
                        window.clear_timeout_with_handle(*timeout_id);
                    }
                    let document = window.document().unwrap();
                    let msg_element = document.get_element_by_id("head-message").unwrap();
                    msg_element.remove_attribute("hidden").ok();

                    let hide_action = {
                        let head_message = head_message.clone();
                        let msg_element = msg_element.clone();
                        let timeout_id = timeout_id.clone();
                        Closure::<dyn Fn()>::new(move || {
                            msg_element.set_attribute("hidden", "true").ok();
                            head_message.set("".to_string());
                            timeout_id.set(-1);
                        })
                    };

                    let id = window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            hide_action.as_ref().unchecked_ref(),
                            2000,
                        )
                        .ok()
                        .unwrap();

                    timeout_id.set(id);
                    hide_action.forget();
                }

                || {}
            },
            head_message,
        )
    }

    html! {
        <>
            <div id="head-message" hidden=true>
                {(*head_message).clone()}
            </div>
                {
                    match &*screen {
                        Screen::Title => {
                            html! {
                                <StartPage
                                    screen={screen.clone()}
                                    head_message={head_message.clone()} />
                            }
                        },
                        Screen::Overview => {
                            html! {
                                <LevelPreviewPage
                                    screen={screen.clone()}
                                    dimension={dimension}
                                    level_number={level_number}/>
                            }
                        },
                        Screen::Editor => {
                            html! {
                                <EditorPage
                                    screen={screen.clone()}
                                    head_message={head_message.clone()}/>
                            }
                        },
                        Screen::Level(user_grid) => {
                            html! {
                                <BoardPage
                                    level_grid={user_grid.clone()}
                                    screen={screen.clone()}
                                    head_message={head_message.clone()}/>

                            }
                        },
                        Screen::Visualizer => {
                            html!{
                                <VisualizerPage
                                    screen={screen.clone()}/>
                            }
                        },
                        Screen::Help => {
                            html!{
                                <TextPage
                                    screen={screen.clone()}
                                    title={"help"}
                                    content={"This is the help page."}/>
                            }
                        },
                        Screen::Credit => {
                            html!{
                                <TextPage
                                    screen={screen.clone()}
                                    title={"credit"}
                                    content={"This is the credit page."}/>
                            }
                        }
                    }
                }
        </>
    }
}
