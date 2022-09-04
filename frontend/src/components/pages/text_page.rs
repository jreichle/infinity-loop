use yew::prelude::*;
use yew::{html, Callback};

use crate::helper::screen::Screen;

#[derive(Properties, PartialEq, Clone)]
pub struct TextPageProps {
    pub screen: UseStateHandle<Screen>,
    pub title: String,
    pub content: Html,
}

/// This page is a simple component that displays given html and a heading.
#[function_component(TextPage)]
pub fn text_page(props: &TextPageProps) -> html {
    let back_onclick: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            screen.set(Screen::Title);
        })
    };

    html! {
        <div class="container">
            <div id={format!("{}{}", props.title.clone(),"-page")} class="page-container">
                <div class="page-title">{format!("#{}", props.title.clone())}</div>
                <div class="page-content">{props.content.clone()}</div>
            </div>
            <div class="controller">
                <button  onclick={back_onclick}>
                    {"-back-"}
                </button>
            </div>
        </div>
    }
}
