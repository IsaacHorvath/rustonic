use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ButtonProps {
    pub kind: String,
    pub text: String
}

#[function_component(Button)]
pub fn button(ButtonProps {kind, text}: &ButtonProps) -> Html {
    let kind = "button ".to_owned() + kind;
    html! {
        <span class={kind}>{text}</span>
    }
}
