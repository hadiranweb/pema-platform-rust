use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or("text".to_string())]
    pub input_type: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let oninput = {
        let onchange = props.onchange.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            onchange.emit(input.value());
        })
    };

    let input_class = format!(
        "input {}{}",
        if props.error.is_some() { "input-error " } else { "" },
        if props.disabled { "input-disabled" } else { "" }
    );

    html! {
        <div class="input-group">
            if let Some(label) = &props.label {
                <label class="input-label">
                    {label}
                    if props.required {
                        <span class="required">{"*"}</span>
                    }
                </label>
            }
            <input
                class={input_class}
                type={props.input_type.clone()}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                required={props.required}
                {oninput}
            />
            if let Some(error) = &props.error {
                <span class="input-error-text">{error}</span>
            }
        </div>
    }
}
