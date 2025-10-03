use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::console;

mod components;
mod pages;
mod services;

use components::header::Header;
use components::footer::Footer;
use pages::{
    home::Home,
    login::Login,
    dashboard::Dashboard,
    products::Products,
    orders::Orders,
    inventory::Inventory,
    vendors::Vendors,
    profile::Profile,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/dashboard")]
    Dashboard,
    #[at("/products")]
    Products,
    #[at("/orders")]
    Orders,
    #[at("/inventory")]
    Inventory,
    #[at("/vendors")]
    Vendors,
    #[at("/profile")]
    Profile,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::Dashboard => html! { <Dashboard /> },
        Route::Products => html! { <Products /> },
        Route::Orders => html! { <Orders /> },
        Route::Inventory => html! { <Inventory /> },
        Route::Vendors => html! { <Vendors /> },
        Route::Profile => html! { <Profile /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let is_authenticated = use_state(|| false);
    let username = use_state(|| None::<String>);

    // Check authentication state on app initialization
    use_effect_with((), {
        let is_authenticated = is_authenticated.clone();
        let username = username.clone();
        
        move |_| {
            console::log_1(&"PEMA Platform Frontend initialized".into());
            
            // Check if user is already authenticated
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(token)) = storage.get_item("auth_token") {
                        if !token.is_empty() {
                            is_authenticated.set(true);
                            if let Ok(Some(stored_username)) = storage.get_item("username") {
                                username.set(Some(stored_username));
                            }
                        }
                    }
                }
            }
            
            || ()
        }
    });

    html! {
        <BrowserRouter>
            <div class="app">
                <Header 
                    is_authenticated={*is_authenticated} 
                    username={(*username).clone()} 
                />
                <main class="main-content">
                    <Switch<Route> render={switch} />
                </main>
                <Footer />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
