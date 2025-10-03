use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::components::{button::Button, input::Input, card::Card};
use crate::services::auth::AuthService;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |value: String| {
            username.set(value);
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |value: String| {
            password.set(value);
        })
    };

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let loading = loading.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            if username.is_empty() || password.is_empty() {
                error.set(Some("لطفاً تمامی فیلدها را پر کنید".to_string()));
                return;
            }

            loading.set(true);
            error.set(None);

            // Use AuthService to authenticate with WASM backend
            let username_val = (*username).clone();
            let password_val = (*password).clone();
            let loading_clone = loading.clone();
            let error_clone = error.clone();
            let navigator_clone = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match AuthService::login(&username_val, &password_val).await {
                    Ok(token) => {
                        // Store token in localStorage or session storage
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("auth_token", &token);
                                let _ = storage.set_item("username", &username_val);
                            }
                        }
                        
                        // Navigate to dashboard
                        navigator_clone.push(&Route::Dashboard);
                    },
                    Err(err_msg) => {
                        error_clone.set(Some(err_msg));
                    }
                }
                
                loading_clone.set(false);
            });
        })
    };

    let on_register_click = {
        Callback::from(move |_| {
            // Navigate to register page or show register modal
            web_sys::console::log_1(&"Register clicked".into());
        })
    };

    html! {
        <div class="login-page">
            <div class="login-container">
                <Card title="ورود به پلتفرم پما" class="login-card">
                    <form onsubmit={on_submit}>
                        <Input
                            label="نام کاربری"
                            value={(*username).clone()}
                            onchange={on_username_change}
                            placeholder="نام کاربری خود را وارد کنید"
                            required=true
                            error={error.as_ref().clone()}
                        />
                        
                        <Input
                            label="رمز عبور"
                            input_type="password"
                            value={(*password).clone()}
                            onchange={on_password_change}
                            placeholder="رمز عبور خود را وارد کنید"
                            required=true
                        />

                        if let Some(err) = error.as_ref() {
                            <div class="error-message">{err}</div>
                        }

                        <div class="login-actions">
                            <Button
                                variant="primary"
                                size="large"
                                loading={*loading}
                                disabled={*loading}
                            >
                                {"ورود"}
                            </Button>
                        </div>
                    </form>

                    <div class="login-footer">
                        <p>{"حساب کاربری ندارید؟"}</p>
                        <button class="link-button" onclick={on_register_click}>
                            {"ثبت‌نام کنید"}
                        </button>
                    </div>

                    <div class="demo-credentials">
                        <h4>{"اطلاعات دمو:"}</h4>
                        <p>{"نام کاربری: admin"}</p>
                        <p>{"رمز عبور: password"}</p>
                    </div>
                </Card>
            </div>
        </div>
    }
}
