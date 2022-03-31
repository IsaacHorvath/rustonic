use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ButtonProps {
    pub kind: String,
    pub text: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(ButtonProps {kind, text, onclick}: &ButtonProps) -> Html {
    let kind = "button ".to_owned() + kind;
    html! {
        <span class={kind} {onclick}>{text}</span>
    }
}
