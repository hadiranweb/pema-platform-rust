use yew::prelude::*;
use yew_router::prelude::*;
use gloo_console::log;
use web_sys::HtmlInputElement;

use crate::components::common::input::Input;
use crate::components::common::button::Button;
use crate::components::common::spinner::Spinner;
use crate::router::routes::AppRoute;
use crate::services::auth_service::{AuthService, LoginRequest};
use crate::state::auth::{AuthStore, Action as AuthAction};
use yewdux::prelude::use_reducer;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let email_state = use_state(|| String::new());
    let password_state = use_state(|| String::new());
    let otp_code_state = use_state(|| Option::<String>::None);
    let error_message_state = use_state(|| Option::<String>::None);
    let loading_state = use_state(|| false);
    let dispatch = use_reducer::<AuthStore>().1;

    let history = use_history().unwrap();

    let on_email_change = Callback::from(move |value: String| {
        
        email_state.set(input.value());
    });

    let on_password_change = Callback::from(move |value: String| {
        
        password_state.set(input.value());
    });

    let on_otp_code_change = Callback::from(move |value: String| {
        
        let value = input.value();
        otp_code_state.set(if value.is_empty() { None } else { Some(value) });
    });

    let on_submit = Callback::from(move |e: FocusEvent| {
        e.prevent_default();
        let email = (*email_state).clone();
        let password = (*password_state).clone();
        let otp_code = (*otp_code_state).clone();
        let error_message_state = error_message_state.clone();
        let loading_state = loading_state.clone();
        let history = history.clone();
        let dispatch = dispatch.clone();

        loading_state.set(true);
        wasm_bindgen_futures::spawn_local(async move {
            let request = LoginRequest { email: email.clone(), password, otp_code };
            match AuthService::login(request).await {
                Ok((token, user_id, username, email)) => {
                    log!("Login successful, token:", token.clone());
                    dispatch.dispatch(AuthAction::Login(token, user_id, username, email));
                    error_message_state.set(None);
                    history.push(AppRoute::Dashboard);
                },
                Err(e) => {
                    log!("Login failed:", e.to_string());
                    error_message_state.set(Some(e.to_string()));
                }
            }
            loading_state.set(false);
        });
    });

    html! {
        <div class="login-page">
            <h1>{ "Login" }</h1>
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
                    label="Password"
                    id="password"
                    type="password"
                    value={(*password_state).clone()}
                    on_change={on_password_change}
                    placeholder="Enter your password"
                />
                <Input
                    label="OTP Code (Optional)"
                    id="otp_code"
                    type="text"
                    value={otp_code_state.as_ref().unwrap_or(&String::new()).clone()}
                    on_change={on_otp_code_change}
                    placeholder="Enter OTP if enabled"
                />
                { if *loading_state { html! { <Spinner /> } } else { html! {} } }
                { if let Some(msg) = &*error_message_state { html! { <p class="error-message">{ msg }</p> } } else { html! {} } }
                <Button label="Login" button_type="submit" />
            </form>
            <p>
                { "Don't have an account? " }
                <Link<AppRoute> to={AppRoute::Register}>{ "Register" }</Link<AppRoute>>
            </p>
        </div>
    }
}

