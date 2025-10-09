use yew::prelude::*;
use yew_router::prelude::*;
use crate::AppRoute;
use crate::components::{Button, Input, Card};
use crate::services::auth::AuthService;
use crate::models::auth::LoginRequest;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| None::<String>);

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |value: String| {
            email.set(value);
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |value: String| {
            password.set(value);
        })
    };

    // Clone handles for the outer Callback closure to be moved into it
    let email_handle_for_submit = email.clone();
    let password_handle_for_submit = password.clone();
    let error_handle_for_submit = error.clone();
    let navigator_handle_for_submit = navigator.clone();

    let on_submit = Callback::from(move |e: web_sys::SubmitEvent| {
        e.prevent_default();
        
        // Clone the values from the UseStateHandles for the async block
        let email_value = (*email_handle_for_submit).clone();
        let password_value = (*password_handle_for_submit).clone();
        let navigator_clone = navigator_handle_for_submit.clone();
        let error_for_async = error_handle_for_submit.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let auth_service = AuthService::new(crate::services::api::ApiService::new("http://localhost:8080/api"));
            let login_request = LoginRequest {
                email: email_value,
                password: password_value,
            };

            match auth_service.login(login_request).await {
                Ok(auth_response) => {
                    crate::services::auth::TokenStorage::save_auth_data(&auth_response);
                    navigator_clone.push(&AppRoute::AdminDashboard);
                }
                Err(err_msg) => {
                    error_for_async.set(Some(err_msg.to_string()));
                }
            }
        });
    });

    html! {
        <div class="login-page">
            <Card title="ورود به پنل مدیریت" class="login-card">
                <form onsubmit={on_submit}>
                    <Input
                        label="ایمیل"
                        input_type="email"
                        placeholder="ایمیل خود را وارد کنید"
                        value={(*email).clone()}
                        onchange={on_email_change}
                        required=true
                    />
                    <Input
                        label="رمز عبور"
                        input_type="password"
                        placeholder="رمز عبور خود را وارد کنید"
                        value={(*password).clone()}
                        onchange={on_password_change}
                        required=true
                    />
                    if let Some(err) = error.as_ref() {
                        <p class="error-message">{err}</p>
                    }
                    <Button button_type="submit" variant="primary" class="login-button">
                        {"ورود"}
                    </Button>
                </form>
            </Card>
        </div>
    }
}

