use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
mod services;
mod state;

use components::{LandingPage, AdminDashboard, DashboardStats};
use services::{ApiService, AuthService};
use services::auth::TokenStorage;
use state::{AppState, AppAction, AppStateContext, AuthAction};

#[derive(Debug, Clone, PartialEq)]
pub enum AppRoute {
    Landing,
    AdminDashboard,
}

#[function_component(App)]
fn app() -> Html {
    let app_state = use_reducer(AppState::default);
    let current_route = use_state(|| AppRoute::Landing);

    // Initialize authentication state from local storage
    use_effect_with((), {
        let app_state = app_state.clone();
        move |_| {
            if let Some(token) = TokenStorage::get_token() {
                if let Some(user) = TokenStorage::get_user() {
                    app_state.dispatch(AppAction::Auth(AuthAction::LoginSuccess(
                        services::auth::AuthResponse {
                            user,
                            token,
                            refresh_token: TokenStorage::get_refresh_token().unwrap_or_default(),
                            expires_in: 3600, // Default expiry
                        }
                    )));
                }
            }
            || ()
        }
    });

    let handle_route_change = {
        let current_route = current_route.clone();
        Callback::from(move |route: AppRoute| {
            current_route.set(route);
        })
    };

    html! {
        <ContextProvider<AppStateContext> context={app_state}>
            {match *current_route {
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
            }}
        </ContextProvider<AppStateContext>>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
