use yew::prelude::*;
use yew::{html, Callback};

use crate::helper::screen::Screen;

#[derive(Properties, PartialEq, Clone)]
pub struct HelpPageProps {
    pub screen: UseStateHandle<Screen>,
}

#[function_component(HelpPage)]
pub fn help_page(props: &HelpPageProps) -> html {
    let back_onclick: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            screen.set(Screen::Title);
        })
    };

    html! {
        <>
            <h1>{"HELP PAGE"}</h1>
            <button  onclick={back_onclick}>
                {"-back-"}
            </button>
        </>
    }
}
