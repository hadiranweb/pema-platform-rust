use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod router;

use router::{switch, Route};
use components::layout::MainLayout;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <MainLayout>
                <Switch<Route> render={switch} />
            </MainLayout>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

