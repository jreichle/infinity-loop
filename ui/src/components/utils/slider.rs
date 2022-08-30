use web_sys::HtmlInputElement;
use yew::html;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SliderComponentProps {
    pub id: String,
    #[prop_or("slider-label".to_string())]
    pub label: String,
    #[prop_or(25)]
    pub max: isize,
    #[prop_or(3)]
    pub min: isize,
    #[prop_or(use_state(|| 14))]
    pub value: UseStateHandle<isize>,
}

#[function_component(SliderComponent)]
pub fn slider_component(props: &SliderComponentProps) -> Html {
    let slider_input_ref = use_node_ref();
    let slider_value = props.value.clone();

    let slider_oninput = {
        let slider_input_ref = slider_input_ref.clone();
        let slider_value = slider_value.clone();

        Callback::from(move |_| {
            if let Some(slider_input) = slider_input_ref.cast::<HtmlInputElement>() {
                slider_value.set(slider_input.value_as_number() as isize);
            }
        })
    };

    html! {
        <div>
            <div class="slider-container">
                <div class="slider-text">
                    <lable for={props.id.clone()} class="slider-label">{props.label.clone()}</lable>
                    <span class="slider-value">{*slider_value.clone()}</span>
                </div>
                <input type="range"
                    min={props.min.to_string()} max={props.max.to_string()}
                    value={slider_value.to_string()}
                    ref={slider_input_ref}
                    id={props.id.clone()} class="slider"
                    oninput={slider_oninput}
                />
            </div>
        </div>
    }
}
