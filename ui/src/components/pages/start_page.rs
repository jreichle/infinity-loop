use yew::prelude::*;
use yew::{html, Callback};

use crate::helper::local_storage::change_screen;
use crate::helper::screen::Screen;

#[derive(Properties, PartialEq, Clone)]
pub struct StartPageProps {
    pub screen: UseStateHandle<Screen>,
    pub message: UseStateHandle<String>,
}

#[function_component(StartPage)]
pub fn start_page(props: &StartPageProps) -> html {
    let screen = props.screen.clone();

    let to_preview: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            change_screen(screen.clone(), Screen::Overview);
        })
    };

    let to_editor: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            change_screen(screen.clone(), Screen::Editor);
        })
    };

    let to_visualizer: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            change_screen(screen.clone(), Screen::Visualizer);
        })
    };

    let to_help: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            change_screen(screen.clone(), Screen::Help);
        })
    };

    let to_credit: Callback<MouseEvent> = {
        let screen = screen.clone();
        Callback::from(move |_| {
            change_screen(screen.clone(), Screen::Credit);
        })
    };

    let on_exit: Callback<MouseEvent> = {
        let message = props.message.clone();
        Callback::from(move |_| {
            message.set("There is no way out of an infinite loop!".to_string());
        })
    };

    html! {
        <div class="container">
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
        </div>
    }
}
