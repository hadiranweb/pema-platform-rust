use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use std::collections::HashMap;
use serde_json;

pub mod components;
pub mod services;
pub mod state;
pub mod i18n;
pub mod pages;
pub mod models;

use crate::services::auth::TokenStorage;
use crate::state::{AppState, AppAction, AppStateContext, AuthAction};
use crate::i18n::{I18nProvider, Language};
use crate::components::{Header, Footer, Sidebar};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/onboarding")]
    Onboarding,
    #[at("/dashboard")]
    AdminDashboard,
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
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    let app_state = use_reducer(AppState::default);

    // Prepare translations
    let translations = load_translations_from_json();

    // Initialize authentication state from local storage
    use_effect_with((), {
        let app_state = app_state.clone();
        move |_| {
            if let Some(token) = TokenStorage::get_token() {
                if let Some(user) = TokenStorage::get_user() {
                    app_state.dispatch(AppAction::Auth(AuthAction::LoginSuccess(
                        crate::models::auth::AuthResponse {
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

    html! {
        <ContextProvider<AppStateContext> context={app_state}>
            <BrowserRouter>
                <I18nProvider 
                    default_language={Language::Persian}
                    translations={Some(translations)}
                >
                    <AppRouter />
                </I18nProvider>
            </BrowserRouter>
        </ContextProvider<AppStateContext>>
    }
}

#[function_component(AppRouter)]
fn app_router() -> Html {
    let navigator = use_navigator().expect("Navigator not found!");
    let route = use_route::<AppRoute>().expect("Route not found!");

    let handle_route_change = Callback::from(move |r: AppRoute| {
        navigator.push(&r);
    });

    html! {
        <div class="app-container">
            <Header />
            <Sidebar on_route_change={handle_route_change.clone()} />
            <main class="main-content">
                { match route {
                    AppRoute::Home => html! { <pages::home::Home /> },
                    AppRoute::Login => html! { <pages::login::Login /> },
                    AppRoute::Register => html! { <pages::register::Register /> },
                    AppRoute::Onboarding => html! { <pages::onboarding::Onboarding /> },
                    AppRoute::AdminDashboard => html! { <pages::dashboard::Dashboard on_route_change={handle_route_change.clone()} /> },
                    AppRoute::Products => html! { <pages::products::Products /> },
                    AppRoute::Orders => html! { <pages::orders::Orders /> },
                    AppRoute::Inventory => html! { <pages::inventory::Inventory /> },
                    AppRoute::Vendors => html! { <pages::vendors::Vendors /> },
                    AppRoute::Profile => html! { <pages::profile::Profile /> },
                    AppRoute::NotFound => html! { <pages::not_found::NotFound /> },
                }}
            </main>
            <Footer />
        </div>
    }
}

// Helper function to load from JSON (compile-time embedding)
fn load_translations_from_json() -> HashMap<Language, HashMap<String, String>> {
    let mut translations = HashMap::new();
    
    // Embed JSON files at compile time
    let fa_json = include_str!("../../locales/fa.json");
    let en_json = include_str!("../../locales/en.json");
    
    // Parse and flatten
    if let Ok(fa_map) = parse_and_flatten_json(fa_json) {
        translations.insert(Language::Persian, fa_map);
    }
    
    if let Ok(en_map) = parse_and_flatten_json(en_json) {
        translations.insert(Language::English, en_map);
    }
    
    translations
}

// Flatten nested JSON to dot notation
fn parse_and_flatten_json(json: &str) -> Result<HashMap<String, String>, serde_json::Error> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    let mut result = HashMap::new();
    flatten_json_value(&value, String::new(), &mut result);
    Ok(result)
}

fn flatten_json_value(
    value: &serde_json::Value,
    prefix: String,
    result: &mut HashMap<String, String>,
) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                flatten_json_value(val, new_prefix, result);
            }
        }
        serde_json::Value::String(s) => {
            result.insert(prefix, s.clone());
        }
        _ => {}
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}

