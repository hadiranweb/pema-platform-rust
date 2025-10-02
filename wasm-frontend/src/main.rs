use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Welcome to PEMA Platform Frontend (Wasm)" }</h1>
            <p>{ "This is a placeholder for your Wasm-powered frontend." }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

