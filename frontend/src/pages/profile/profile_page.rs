use yew::prelude::*;
use yewdux::prelude::use_selector;
use gloo_console::log;
use crate::state::auth::AuthStore;
use crate::services::auth_service::AuthService;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let auth_state = use_selector(|state: &AuthStore| state.clone());
    let otp_message_state = use_state(|| Option::<String>::None);

    let on_generate_otp = { 
        let auth_state = auth_state.clone();
        let otp_message_state = otp_message_state.clone();
        Callback::from(move |_| {
            let auth_state = auth_state.clone();
            let otp_message_state = otp_message_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(token) = &auth_state.token {
                    match AuthService::generate_otp(token).await {
                        Ok(otp_code) => {
                            log!("Generated OTP:", otp_code.clone());
                            otp_message_state.set(Some(format!("Generated OTP: {}", otp_code)));
                        },
                        Err(e) => {
                            log!("Failed to generate OTP:", e.to_string());
                            otp_message_state.set(Some(format!("Failed to generate OTP: {}", e.to_string())));
                        }
                    }
                } else {
                    otp_message_state.set(Some("You must be logged in to generate OTP.".to_string()));
                }
            });
        })
    };

    html! {
        <div class="profile-page">
            <h1>{ "User Profile" }</h1>
            { if let Some(username) = &auth_state.username {
                html! { <p>{ format!("Username: {}", username) }</p> }
            } else {
                html! { <p>{ "Username: N/A" }</p> }
            } }
            { if let Some(email) = &auth_state.email {
                html! { <p>{ format!("Email: {}", email) }</p> }
            } else {
                html! { <p>{ "Email: N/A" }</p> }
            } }
            { if let Some(user_id) = &auth_state.user_id {
                html! { <p>{ format!("User ID: {}", user_id) }</p> }
            } else {
                html! { <p>{ "User ID: N/A" }</p> }
            } }
            <button onclick={on_generate_otp}>{ "Generate OTP" }</button>
            { if let Some(msg) = &*otp_message_state { html! { <p>{ msg }</p> } } else { html! {} } }
            // Additional profile details can be added here
        </div>
    }
}

