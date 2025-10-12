use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div>
            <h1>{ "Welcome to PEMA Platform" }</h1>
            <p>{ "Your one-stop shop for everything." }</p>
        </div>
    }
}

