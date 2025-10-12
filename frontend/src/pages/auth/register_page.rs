
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_console::log;
use web_sys::HtmlInputElement;

use crate::components::common::input::Input;
use crate::components::common::button::Button;
use crate::components::common::spinner::Spinner;
use crate::router::routes::AppRoute;
use crate::services::auth_service::{AuthService, RegisterRequest};
use crate::state::auth::{AuthStore, Action as AuthAction};
use yewdux::prelude::use_reducer_dispatch;

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let email_state = use_state(|| String::new());
    let username_state = use_state(|| String::new());
    let password_state = use_state(|| String::new());
    let error_message_state = use_state(|| Option::<String>::None);
    let loading_state = use_state(|| false);

    let history = use_history().unwrap();

    let on_email_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        email_state.set(input.value());
    });

    let on_username_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        username_state.set(input.value());
    });

    let on_password_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        password_state.set(input.value());
    });

    let on_submit = Callback::from(move |e: FocusEvent| {
        e.prevent_default();
        let email = (*email_state).clone();
        let username = (*username_state).clone();
        let password = (*password_state).clone();
        let error_message_state = error_message_state.clone();
        let loading_state = loading_state.clone();
        let history = history.clone();
        let dispatch = use_reducer_dispatch::<AuthStore>();

        loading_state.set(true);
        wasm_bindgen_futures::spawn_local(async move {
            let request = RegisterRequest { email, username, password };
            match AuthService::register(request).await {
                Ok((token, user_id, username, email)) => {
                    log!("Registration successful!");
                    dispatch.dispatch(AuthAction::Login(token, user_id, username, email));
                    error_message_state.set(None);
                    history.push(AppRoute::Dashboard);
                },
                Err(e) => {
                    log!("Registration failed:", e.to_string());
                    error_message_state.set(Some(e.to_string()));
                }
            }
            loading_state.set(false);
        });
    });

    html! {
        <div class="register-page">
            <h1>{ "Register" }</h1>
            <form onsubmit={on_submit}>
                <Input
                    label="Email"
                    id="email"
                    type="email"
                    value={(*email_state).clone()}
                    on_change={on_email_change}
                    placeholder="Enter your email"
                />
                <Input
                    label="Username"
                    id="username"
                    type="text"
                    value={(*username_state).clone()}
                    on_change={on_username_change}
                    placeholder="Choose a username"
                />
                <Input
                    label="Password"
                    id="password"
                    type="password"
                    value={(*password_state).clone()}
                    on_change={on_password_change}
                    placeholder="Choose a password"
                />
                { if *loading_state { html! { <Spinner /> } } else { html! {} } }
                { if let Some(msg) = &*error_message_state { html! { <p class="error-message">{ msg }</p> } } else { html! {} } }
                <Button label="Register" button_type="submit" />
            </form>
            <p>
                { "Already have an account? " }
                <Link<AppRoute> to={AppRoute::Login}>{ "Login" }</Link<AppRoute>>
            </p>
        </div>
    }
}

