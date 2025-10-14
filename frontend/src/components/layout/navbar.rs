use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::routes::AppRoute;
use crate::state::auth::AuthStore;
use yewdux::prelude::use_store;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let (auth_state, _) = use_store::<AuthStore>();

    html! {
        <nav class="navbar">
            <ul class="navbar-list">
                <li class="navbar-item"><Link<AppRoute> to={AppRoute::Home}>{ "Home" }</Link<AppRoute>></li>
                <li class="navbar-item"><Link<AppRoute> to={AppRoute::Products}>{ "Products" }</Link<AppRoute>></li>
                { if auth_state.token.is_some() {
                    html! {
                        <>
                            <li class="navbar-item"><Link<AppRoute> to={AppRoute::Orders}>{ "Orders" }</Link<AppRoute>></li>
                            <li class="navbar-item"><Link<AppRoute> to={AppRoute::Wallet}>{ "Wallet" }</Link<AppRoute>></li>
                            <li class="navbar-item"><Link<AppRoute> to={AppRoute::Reviews}>{ "Reviews" }</Link<AppRoute>></li>
                            <li class="navbar-item"><Link<AppRoute> to={AppRoute::Profile}>{ "Profile" }</Link<AppRoute>></li>
                            // Admin and Vendor links would typically be conditionally rendered based on user roles
                            // For now, assuming they are always visible if logged in.
                            <li class="navbar-item"><Link<AppRoute> to={AppRoute::AdminDashboard}>{ "Admin" }</Link<AppRoute>></li>
                            <li class="navbar-item"><Link<AppRoute> to={AppRoute::VendorDashboard}>{ "Vendor" }</Link<AppRoute>></li>
                        </>
                    }
                } else {
                    html! {}
                }}
            </ul>
        </nav>
    }
}

