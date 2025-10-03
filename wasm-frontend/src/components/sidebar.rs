use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    #[prop_or(false)]
    pub is_open: bool,
    #[prop_or_default]
    pub onclose: Callback<()>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let class = if props.is_open {
        "sidebar sidebar-open"
    } else {
        "sidebar"
    };

    html! {
        <aside class={class}>
            <div class="sidebar-header">
                <h3>{"منو"}</h3>
                <button class="sidebar-close" onclick={props.onclose.clone()}>
                    {"×"}
                </button>
            </div>
            
            <nav class="sidebar-nav">
                <Link<Route> to={Route::Dashboard} classes="sidebar-link">
                    {"داشبورد"}
                </Link<Route>>
                <Link<Route> to={Route::Products} classes="sidebar-link">
                    {"محصولات"}
                </Link<Route>>
                <Link<Route> to={Route::Orders} classes="sidebar-link">
                    {"سفارشات"}
                </Link<Route>>
                <Link<Route> to={Route::Inventory} classes="sidebar-link">
                    {"موجودی"}
                </Link<Route>>
                <Link<Route> to={Route::Vendors} classes="sidebar-link">
                    {"تأمین‌کنندگان"}
                </Link<Route>>
                <Link<Route> to={Route::Profile} classes="sidebar-link">
                    {"پروفایل"}
                </Link<Route>>
            </nav>
        </aside>
    }
}
