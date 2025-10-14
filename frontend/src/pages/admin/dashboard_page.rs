use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::routes::AppRoute;

#[function_component(AdminDashboardPage)]
pub fn admin_dashboard_page() -> Html {
    html! {
        <div class="admin-dashboard-page">
            <h1>{ "Admin Dashboard" }</h1>
            <p>{ "Welcome to the admin dashboard. Here you can manage users, products, orders, vendors, and pages." }</p>
            <nav>
                <ul>
                    <li><Link<AppRoute> to={AppRoute::AdminUsers}>{ "Manage Users" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::AdminProducts}>{ "Manage Products" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::AdminOrders}>{ "Manage Orders" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::AdminVendors}>{ "Manage Vendors" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::AdminPages}>{ "Manage Pages" }</Link<AppRoute>></li>
                </ul>
            </nav>
        </div>
    }
}
