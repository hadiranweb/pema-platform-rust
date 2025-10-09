use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::services::auth::AuthService;

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
        Callback::from(move |_| {
            navigator.push(&Route::Login);
        })
    };

    let on_dashboard_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Dashboard);
        })
    };

    let on_home_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };

    let on_logout_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            // Clear authentication state
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.remove_item("auth_token");
                    let _ = storage.remove_item("username");
                }
            }
            
            // Call AuthService logout
            let _ = AuthService::logout();
            
            // Navigate to home
            navigator.push(&Route::Home);
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
                    <Link<Route> to={Route::Home} classes="nav-link">{"خانه"}</Link<Route>>
                    <Link<Route> to={Route::Products} classes="nav-link">{"محصولات"}</Link<Route>>
                    if props.is_authenticated {
                        <Link<Route> to={Route::Dashboard} classes="nav-link">{"داشبورد"}</Link<Route>>
                        <Link<Route> to={Route::Orders} classes="nav-link">{"سفارشات"}</Link<Route>>
                        <Link<Route> to={Route::Inventory} classes="nav-link">{"موجودی"}</Link<Route>>
                        <Link<Route> to={Route::Vendors} classes="nav-link">{"تأمین‌کنندگان"}</Link<Route>>
                    }
                </nav>

                <div class="auth-section">
                    if props.is_authenticated {
                        <div class="user-menu">
                            <span class="username">{props.username.as_ref().unwrap_or(&"کاربر".to_string())}</span>
                            <Link<Route> to={Route::Profile} classes="profile-link">{"پروفایل"}</Link<Route>>
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
