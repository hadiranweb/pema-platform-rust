use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
use components::{LandingPage, AdminDashboard, DashboardStats};

#[derive(Debug, Clone, PartialEq)]
pub enum AppRoute {
    Landing,
    AdminDashboard,
}

#[function_component(App)]
fn app() -> Html {
    let current_route = use_state(|| AppRoute::Landing);

    let handle_route_change = {
        let current_route = current_route.clone();
        Callback::from(move |route: AppRoute| {
            current_route.set(route);
        })
    };

    match *current_route {
        AppRoute::Landing => html! {
            <div>
                <LandingPage />
                <div class="fixed bottom-4 right-4">
                    <button 
                        onclick={handle_route_change.reform(|_| AppRoute::AdminDashboard)}
                        class="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded-lg shadow-lg transition-colors"
                    >
                        {"داشبورد مدیریت"}
                    </button>
                </div>
            </div>
        },
        AppRoute::AdminDashboard => html! {
            <div>
                <AdminDashboard stats={DashboardStats::default()} />
                <div class="fixed bottom-4 right-4">
                    <button 
                        onclick={handle_route_change.reform(|_| AppRoute::Landing)}
                        class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg shadow-lg transition-colors"
                    >
                        {"صفحه اصلی"}
                    </button>
                </div>
            </div>
        },
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
