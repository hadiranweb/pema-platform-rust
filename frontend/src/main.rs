use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod router;
mod config;

use router::{switch, Route};
use components::layout::MainLayout;
use config::FrontendConfig;

#[function_component(App)]
fn app() -> Html {
    let config = use_state(|| FrontendConfig::new());

    html! {
        <ContextProvider<FrontendConfig> context={(*config).clone()}>
            <BrowserRouter>
                <MainLayout>
                    <Switch<Route> render={switch} />
                </MainLayout>
            </BrowserRouter>
        </ContextProvider<FrontendConfig>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

