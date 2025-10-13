use yew::prelude::*;

use crate::AppRoute;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    #[prop_or(false)]
    pub is_open: bool,
    #[prop_or_default]
    pub onclose: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_route_change: Callback<AppRoute>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let class = if props.is_open {
        "sidebar sidebar-open"
    } else {
        "sidebar"
    };

    let on_dashboard_click = {
        let on_route_change = props.on_route_change.clone();
        Callback::from(move |_| on_route_change.emit(AppRoute::AdminDashboard))
    };

    let on_products_click = {
        let on_route_change = props.on_route_change.clone();
        Callback::from(move |_| on_route_change.emit(AppRoute::Products))
    };

    let on_orders_click = {
        let on_route_change = props.on_route_change.clone();
        Callback::from(move |_| on_route_change.emit(AppRoute::Orders))
    };

    let on_inventory_click = {
        let on_route_change = props.on_route_change.clone();
        Callback::from(move |_| on_route_change.emit(AppRoute::Inventory))
    };

    let on_vendors_click = {
        let on_route_change = props.on_route_change.clone();
        Callback::from(move |_| on_route_change.emit(AppRoute::Vendors))
    };

    let on_profile_click = {
        let on_route_change = props.on_route_change.clone();
        Callback::from(move |_| on_route_change.emit(AppRoute::Profile))
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
                <button class="sidebar-link" onclick={on_dashboard_click}>
                    {"داشبورد"}
                </button>
                <button class="sidebar-link" onclick={on_products_click}>
                    {"محصولات"}
                </button>
                <button class="sidebar-link" onclick={on_orders_click}>
                    {"سفارشات"}
                </button>
                <button class="sidebar-link" onclick={on_inventory_click}>
                    {"موجودی"}
                </button>
                <button class="sidebar-link" onclick={on_vendors_click}>
                    {"تأمین‌کنندگان"}
                </button>
                <button class="sidebar-link" onclick={on_profile_click}>
                    {"پروفایل"}
                </button>
            </nav>
        </aside>
    }
}

