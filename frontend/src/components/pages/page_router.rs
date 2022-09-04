use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::pages::board_page::BoardPage;
use crate::components::pages::editor_page::EditorPage;
use crate::components::pages::level_preview::LevelPreviewPage;
use crate::components::pages::start_page::StartPage;
use crate::components::pages::text_page::TextPage;
use crate::components::pages::visualizer_page::VisualizerPage;

use crate::helper::local_storage::{change_screen, retrieve_screen};
use crate::helper::screen::Screen;

use game::model::coordinate::Coordinate;

/// This page is used as a router which directs to the correct page depending on
/// which screen is required
/// Additionally the page can display messages to inform the user.
#[function_component(PageRouter)]
pub fn page_router() -> Html {
    let head_message = use_state_eq(|| "".to_string());
    let bottom_message = use_state_eq(|| "".to_string());
    let head_message_timeout_id = use_state(|| -1_i32);
    let bottom_message_timeout_id = use_state(|| -2_i32);

    let dimension = use_state(|| Coordinate::new(5_usize, 5_usize));
    let screen = use_state(retrieve_screen);

    let to_title: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            log::info!("[Button click] Editor");
            change_screen(screen.clone(), Screen::Title);
        })
    };

    // use effect to let head_message disappear after 1.5 seconds and let bottom_message disappear after 10 seconds
    // depends on message change
    {
        let head_message = head_message.clone();
        let timeout_id = head_message_timeout_id;
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
                        let msg_element = msg_element;
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
        );

        let bottom_message = bottom_message.clone();
        let timeout_id = bottom_message_timeout_id;
        use_effect_with_deps(
            move |message| {
                let window = web_sys::window().unwrap();
                let message_string = (*message.clone()).clone();
                if !message_string.is_empty() {
                    if *timeout_id != -1 {
                        window.clear_timeout_with_handle(*timeout_id);
                    }
                    let document = window.document().unwrap();
                    let msg_element = document.get_element_by_id("bottom-message").unwrap();
                    msg_element.remove_attribute("hidden").ok();

                    let hide_action = {
                        let message = message.clone();
                        let msg_element = msg_element;
                        let timeout_id = timeout_id.clone();
                        Closure::<dyn Fn()>::new(move || {
                            msg_element.set_attribute("hidden", "true").ok();
                            message.set("".to_string());
                            timeout_id.set(-2);
                        })
                    };

                    let id = window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            hide_action.as_ref().unchecked_ref(),
                            10000,
                        )
                        .ok()
                        .unwrap();

                    timeout_id.set(id);
                    hide_action.forget();
                }

                || {}
            },
            bottom_message,
        )
    }

    html! {
        <>
            <div
                id="title"
                onclick={to_title}>
                {"Rusty infinity loop!"}
            </div>
            <div id="head-message" hidden=true>
                {(*head_message).clone()}
            </div>
                {
                    match &*screen {
                        Screen::Title => {
                            html! {
                                <StartPage
                                    screen={screen.clone()}
                                    head_message={head_message} />
                            }
                        },
                        Screen::Overview => {
                            html! {
                                <LevelPreviewPage
                                    screen={screen.clone()}
                                    dimension={dimension}/>
                            }
                        },
                        Screen::Editor => {
                            html! {
                                <EditorPage
                                    screen={screen.clone()}
                                    head_message={head_message}/>
                            }
                        },
                        Screen::Level(user_grid) => {
                            html! {
                                <BoardPage
                                    level_grid={user_grid.clone()}
                                    screen={screen.clone()}
                                    head_message={head_message}
                                    cnf={bottom_message.clone()}
                                    literals={use_state_eq(|| "".to_string())}/>

                            }
                        },
                        Screen::Visualizer => {
                            html!{
                                <VisualizerPage
                                    screen={screen.clone()}/>
                            }
                        },
                        Screen::Help => {
                            let content = html!{
                                <>
                                    <p>
                                        {"In the programming world, an infinite loop is a
                                        sequence of instructions that will repeat endlessly.
                                        Usually we would want to prevent this, but in this
                                        game we want to create a loop with no way out."}
                                    </p>
                                    <p>
                                        {"A level is build with multiple cells, each cell
                                        contains a fixed shape that can be rotated on click.
                                        And there are six kind of shape in total. (from none
                                        to four outgoing connections) The goal is to get all
                                        the piece connected, creating a enclosed loop."}
                                    </p>
                                </>
                            };
                            html!{
                                <TextPage
                                    screen={screen.clone()}
                                    title={"help"}
                                    content={content} />
                            }
                        },
                        Screen::Credit => {
                            let content = html!{
                                <>
                                <p>{"inspired by the android game: ["}
                                <a href={"https://play.google.com/store/apps/details?id=com.balysv.loop"} target={"_blank"}>
                                {"Infinity Loop"}</a>{"]"}
                                </p>
                                <p>{"developed by: *"}</p>
                                <p>{"Jakob Ritter"}</p>
                                <p>{"Jan Alexander Jensen"}</p>
                                <p>{"Johannes Moosburger"}</p>
                                <p>{"Johannes Reichle"}</p>
                                <p>{"Simon Redl"}</p>
                                <p>{"* in alphabetical order"}</p>
                                </>
                            };
                            html!{
                                <TextPage
                                    screen={screen.clone()}
                                    title={"credits"}
                                    content={content} />
                            }
                        }
                    }
                }
                <div id="bottom-message" hidden=false>
                {(*bottom_message).clone()}
            </div>
        </>
    }
}
