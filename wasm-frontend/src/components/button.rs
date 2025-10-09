use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or("primary".to_string())]
    pub variant: String,
    #[prop_or("medium".to_string())]
    pub size: String,
    #[prop_or_default]
    pub loading: bool,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = format!(
        "btn btn-{} btn-{} {}{}",
        props.variant,
        props.size,
        if props.disabled { "btn-disabled " } else { "" },
        if props.loading { "btn-loading" } else { "" }
    );

    html! {
        <button
            class={class}
            onclick={props.onclick.clone()}
            disabled={props.disabled || props.loading}
        >
            if props.loading {
                <span class="loading-spinner"></span>
            }
            { for props.children.iter() }
        </button>
    }
}
