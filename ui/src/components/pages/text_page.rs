use yew::prelude::*;
use yew::{html, Callback};

use crate::helper::screen::Screen;

#[derive(Properties, PartialEq, Clone)]
pub struct TextPageProps {
    pub screen: UseStateHandle<Screen>,
    pub title: String,
    pub content: String,
}

#[function_component(TextPage)]
pub fn Text_page(props: &TextPageProps) -> html {
    let back_onclick: Callback<MouseEvent> = {
        let screen = props.screen.clone();
        Callback::from(move |_| {
            screen.set(Screen::Title);
        })
    };

    html!{
        <>
            <div id={format!("{}{}", props.title.clone(),"-page")} class="page-container">
                <div class="page-title">{format!("#{}", props.title.clone())}</div>
                <div class="page-content">{props.content.clone()}</div>
            </div>
            <div id="controller">
                <button  onclick={back_onclick}>
                    {"-back-"}
                </button>
            </div>
        </>
    }
}