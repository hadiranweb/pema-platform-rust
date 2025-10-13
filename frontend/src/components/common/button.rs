use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub button_type: String,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            class={format!("{} btn", props.class)}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
            type={if props.button_type.is_empty() { "button".to_string() } else { props.button_type.clone() }}
        >
            if !props.label.is_empty() {
                { &props.label }
            } else {
                { props.children.clone() }
            }
        </button>
    }
}

