use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="loading-indicator">
            <p>{ "Loading..." }</p>
        </div>
    }
}

