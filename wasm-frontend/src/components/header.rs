use yew::prelude::*;
use yew_router::prelude::*;
use crate::AppRoute;
use crate::services::auth::AuthService;
use wasm_bindgen_futures;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub is_authenticated: bool,
    #[prop_or_default]
    pub username: Option<String>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let navigator = use_navigator().unwrap();

    let on_login_click = {
        let navigator = navigator.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            navigator.push(&AppRoute::Login);
        })
    };



    let on_home_click = {
        let navigator = navigator.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            navigator.push(&AppRoute::Home);
        })
    };

    let on_logout_click = {
        let navigator = navigator.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            // Clear authentication state
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.remove_item("auth_token");
                    let _ = storage.remove_item("username");
                }
            }
            
            // Call AuthService logout
            let auth_service = AuthService::default();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = auth_service.logout().await;
            });
            
            // Navigate to home
            navigator.push(&AppRoute::Home);
        })
    };

    html! {
        <header class="header">
            <div class="header-container">
                <div class="logo" onclick={on_home_click}>
                    <h1>{"PEMA Platform"}</h1>
                    <span class="subtitle">{"نقره‌های ارزشمند"}</span>
                </div>
                
                <nav class="nav">
                    <Link<AppRoute> to={AppRoute::Home} classes="nav-link">{"خانه"}</Link<AppRoute>>
                    <Link<AppRoute> to={AppRoute::Products} classes="nav-link">{"محصولات"}</Link<AppRoute>>
                    if props.is_authenticated {
                        <Link<AppRoute> to={AppRoute::AdminDashboard} classes="nav-link">{"داشبورد"}</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Orders} classes="nav-link">{"سفارشات"}</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Inventory} classes="nav-link">{"موجودی"}</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Vendors} classes="nav-link">{"تأمین‌کنندگان"}</Link<AppRoute>>
                    }
                </nav>

                <div class="auth-section">
                    if props.is_authenticated {
                        <div class="user-menu">
                            <span class="username">{props.username.as_ref().unwrap_or(&"کاربر".to_string())}</span>
                            <Link<AppRoute> to={AppRoute::Profile} classes="profile-link">{"پروفایل"}</Link<AppRoute>>
                            <button class="logout-btn" onclick={on_logout_click}>{"خروج"}</button>
                        </div>
                    } else {
                        <button class="login-btn" onclick={on_login_click}>{"ورود"}</button>
                    }
                </div>
            </div>
        </header>
    }
}
