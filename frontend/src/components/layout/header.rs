use yew::prelude::*;
use yew_router::prelude::Link;
use crate::router::routes::AppRoute;
use crate::state::auth::{AuthStore, Action as AuthAction};
use yewdux::prelude::use_reducer_dispatch;

#[function_component(Header)]
pub fn header() -> Html {
    let (auth_state, _) = use_store::<AuthStore>();
    let dispatch = use_reducer_dispatch::<AuthStore>();

    let on_logout = Callback::from(move |_| {
        dispatch.dispatch(AuthAction::Logout);
    });

    html! {
        <header class="app-header">
            <h1><Link<AppRoute> to={AppRoute::Home}>{ "PEMA Platform" }</Link<AppRoute>></h1>
            <nav>
                { if auth_state.token.is_some() {
                    html! {
                        <div class="user-info">
                            <span>{ format!("Welcome, {}!", auth_state.username.clone().unwrap_or_default()) }</span>
                            <button onclick={on_logout}>{ "Logout" }</button>
                        </div>
                    }
                } else {
                    html! {
                        <div class="auth-links">
                            <Link<AppRoute> to={AppRoute::Login}>{ "Login" }</Link<AppRoute>>
                            <Link<AppRoute> to={AppRoute::Register}>{ "Register" }</Link<AppRoute>>
                        </div>
                    }
                }}
            </nav>
        </header>
    }
}

