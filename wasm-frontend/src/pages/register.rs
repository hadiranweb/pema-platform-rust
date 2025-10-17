use yew::prelude::*;
use yew_router::prelude::*;
use crate::AppRoute;
use crate::components::{Button, Input, Card, AnimatedSkyBackground, PemaMoon};
use crate::services::auth::AuthService;
use crate::services::auth::RegisterRequest;

#[function_component(Register)]
pub fn register() -> Html {
    let navigator = use_navigator().unwrap();
    let email = use_state(|| String::new());
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let confirm_password = use_state(|| String::new());
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |value: String| {
            email.set(value);
        })
    };

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

    let on_confirm_password_change = {
        let confirm_password = confirm_password.clone();
        Callback::from(move |value: String| {
            confirm_password.set(value);
        })
    };

    let on_submit = {
        let email = email.clone();
        let username = username.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();
        let error = error.clone();
        let loading = loading.clone();
        let navigator = navigator.clone();
        
        Callback::from(move |e: web_sys::SubmitEvent| {
            e.prevent_default();
            
            // Validation
            if (*password).trim().is_empty() || (*email).trim().is_empty() || (*username).trim().is_empty() {
                error.set(Some("لطفاً همه فیلدها را پر کنید".to_string()));
                return;
            }
            
            if *password != *confirm_password {
                error.set(Some("رمز عبور و تکرار آن یکسان نیستند".to_string()));
                return;
            }
            
            if (*password).len() < 6 {
                error.set(Some("رمز عبور باید حداقل ۶ کاراکتر باشد".to_string()));
                return;
            }
            
            let email_value = (*email).clone();
            let username_value = (*username).clone();
            let password_value = (*password).clone();
            let navigator_clone = navigator.clone();
            let error_for_async = error.clone();
            let loading_for_async = loading.clone();

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let auth_service = AuthService::new(crate::services::api::ApiService::new("http://localhost:8080/api"));
                let register_request = RegisterRequest {
                    email: email_value,
                    name: username_value,
                    password: password_value,
                    phone: None,
                };

                match auth_service.register(register_request).await {
                    Ok(auth_response) => {
                        crate::services::auth::TokenStorage::save_auth_data(&auth_response);
                        navigator_clone.push(&AppRoute::AdminDashboard);
                    }
                    Err(err_msg) => {
                        error_for_async.set(Some(format!("خطا در ثبت‌نام: {}", err_msg)));
                    }
                }
                loading_for_async.set(false);
            });
        })
    };

    let on_login_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&AppRoute::Login);
        })
    };

    html! {
        <div class="register-page">
            <AnimatedSkyBackground />
            <PemaMoon class="register-moon" />
            
            <div class="register-container">
                <Card title="ثبت‌نام در پلتفرم پما" class="register-card">
                    <div class="register-header">
                        <h1 class="register-title">{"عضویت در پلتفرم پما"}</h1>
                        <p class="register-subtitle">{"سفر سرمایه‌گذاری هوشمند خود را آغاز کنید"}</p>
                    </div>
                    
                    <form onsubmit={on_submit} class="register-form">
                        <div class="form-row">
                            <Input
                                label="نام کاربری"
                                input_type="text"
                                placeholder="نام کاربری خود را وارد کنید"
                                value={(*username).clone()}
                                onchange={on_username_change}
                                required=true
                                class="register-input"
                            />
                        </div>
                        
                        <div class="form-row">
                            <Input
                                label="ایمیل"
                                input_type="email"
                                placeholder="ایمیل خود را وارد کنید"
                                value={(*email).clone()}
                                onchange={on_email_change}
                                required=true
                                class="register-input"
                            />
                        </div>
                        
                        <div class="form-row">
                            <Input
                                label="رمز عبور"
                                input_type="password"
                                placeholder="رمز عبور خود را وارد کنید"
                                value={(*password).clone()}
                                onchange={on_password_change}
                                required=true
                                class="register-input"
                            />
                        </div>
                        
                        <div class="form-row">
                            <Input
                                label="تکرار رمز عبور"
                                input_type="password"
                                placeholder="رمز عبور را مجدداً وارد کنید"
                                value={(*confirm_password).clone()}
                                onchange={on_confirm_password_change}
                                required=true
                                class="register-input"
                            />
                        </div>
                        
                        if let Some(err) = error.as_ref() {
                            <div class="error-message register-error">
                                {err}
                            </div>
                        }
                        
                        <div class="register-actions">
                            <Button 
                                button_type="submit" 
                                variant="primary" 
                                size="large"
                                class="register-button"
                                disabled={*loading}
                            >
                                {if *loading { "در حال ثبت‌نام..." } else { "ثبت‌نام" }}
                            </Button>
                        </div>
                    </form>
                    
                    <div class="register-footer">
                        <p class="login-link">
                            {"قبلاً عضو شده‌اید؟ "}
                            <button 
                                type="button" 
                                class="link-button" 
                                onclick={on_login_click}
                            >
                                {"وارد شوید"}
                            </button>
                        </p>
                    </div>
                </Card>
            </div>
        </div>
    }
}