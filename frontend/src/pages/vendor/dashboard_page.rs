
use yew::prelude::*;

#[function_component(VendorDashboardPage)]
pub fn vendor_dashboard_page() -> Html {
    html! {
        <div class="vendor-dashboard-page">
            <h1>{ "Vendor Dashboard" }</h1>
            <p>{ "Welcome to your vendor dashboard. Here you can manage your products and orders." }</p>
            // Add links to products and orders management
        </div>
    }
}

