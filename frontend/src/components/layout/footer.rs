use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="app-footer">
            <p>{ "© 2023 PEMA Platform" }</p>
        </footer>
    }
}

