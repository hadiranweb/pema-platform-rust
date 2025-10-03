use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
use components::LandingPage;

#[function_component(App)]
fn app() -> Html {
    html! {
        <LandingPage />
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
