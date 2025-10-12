use yew::prelude::*;
use yew_router::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::services::order_service::{OrderService, Order};
use crate::router::routes::AppRoute;

#[function_component(VendorOrdersPage)]
pub fn vendor_orders_page() -> Html {
    let orders_state = use_state(|| None::<Vec<Order>>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);

    { // Fetch vendor orders on component mount
        let orders_state = orders_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Assuming there's a service method to get vendor-specific orders
                // For now, we'll use get_all_orders as a placeholder or implement a new one.
                // In a real scenario, the backend would filter orders by vendor_id.
                match OrderService::get_all_orders().await {
                    Ok(orders) => {
                        orders_state.set(Some(orders));
                        loading_state.set(false);
                    },
                    Err(e) => {
                        log!("Failed to fetch vendor orders:", e.to_string());
                        error_state.set(Some(e.to_string()));
                        loading_state.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="vendor-orders-page">
            <h1>{ "Vendor Orders" }</h1>
            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            <div class="order-list">
                { 
                    if let Some(orders) = &*orders_state {
                        if orders.is_empty() {
                            html! { <p>{ "No orders for your products yet." }</p> }
                        } else {
                            orders.iter().map(|order| {
                                let order_id = order.id.clone();
                                html! {
                                    <div key={order.id.to_string()} class="order-card">
                                        <h3><Link<AppRoute> to={AppRoute::OrderDetail { id: order_id }}>{ format!("Order # {}", order.id) }</Link<AppRoute>></h3>
                                        <p>{ format!("Status: {}", order.status) }</p>
                                        <p>{ format!("Total: ${:.2}", order.total_amount) }</p>
                                        // Add more order details specific to vendor view
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    } else {
                        html! { <p>{ "Loading orders..." }</p> }
                    }
                }
            }
        </div>
    }
}

