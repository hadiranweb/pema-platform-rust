use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub input_type: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or(false)]
    pub required: bool,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let onchange = props.onchange.clone();
    let handle_onchange = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        onchange.emit(value);
    });

    html! {
        <div class={format!("form-group {}", props.class)}>
            if !props.label.is_empty() {
                <label>{ &props.label }</label>
            }
            <input
                type={props.input_type.clone()}
                value={props.value.clone()}
                onchange={handle_onchange}
                placeholder={props.placeholder.clone()}
                required={props.required}
                class="form-control"
            />
        </div>
    }
}

