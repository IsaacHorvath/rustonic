use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or("".to_string())]
    pub id: String,
    pub kind: String,
    pub text: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(ButtonProps {id, kind, text, onclick}: &ButtonProps) -> Html {
    let kind = "button ".to_owned() + kind;
    html! {
        <span id={id.clone()} class={kind} {onclick}>{text}</span>
    }
}
