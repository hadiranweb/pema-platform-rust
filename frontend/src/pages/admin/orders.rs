use yew::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::services::admin_service::{AdminService, Order, AdminOrderUpdateDto};

#[function_component(AdminOrdersPage)]
pub fn admin_orders_page() -> Html {
    let orders_state = use_state(|| None::<Vec<Order>>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);

    let fetch_orders = { 
        let orders_state = orders_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        Callback::from(move |_| {
            let orders_state = orders_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_state.set(true);
                match AdminService::get_all_orders().await {
                    Ok(orders) => {
                        orders_state.set(Some(orders));
                        error_state.set(None);
                    },
                    Err(e) => {
                        log!("Failed to fetch orders:", e.to_string());
                        error_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    use_effect_with_deps(move |fetch_orders| {
        fetch_orders.emit(());
        || ()
    }, fetch_orders.clone());

    let on_delete_order = { 
        let fetch_orders = fetch_orders.clone();
        Callback::from(move |order_id: String| {
            let fetch_orders = fetch_orders.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match AdminService::delete_order(order_id).await {
                    Ok(_) => {
                        log!("Order deleted successfully");
                        fetch_orders.emit(());
                    },
                    Err(e) => {
                        log!("Failed to delete order:", e.to_string());
                    }
                }
            });
        })
    };

    // Placeholder for update functionality - could use a modal with a form
    let on_update_order = Callback::from(move |order: Order| {
        log!("Update order:", order.id.to_string());
        // Implement modal or navigation to an edit page
    });

    html! {
        <div class="admin-orders-page">
            <h1>{ "Manage Orders" }</h1>
            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            <div class="order-list">
                { 
                    if let Some(orders) = &*orders_state {
                        if orders.is_empty() {
                            html! { <p>{ "No orders found." }</p> }
                        } else {
                            orders.iter().map(|order| {
                                let order_id = order.id.to_string();
                                let order_clone = order.clone();
                                let on_delete = on_delete_order.clone();
                                let on_update = on_update_order.clone();
                                html! {
                                    <div key={order_id.clone()} class="order-card">
                                        <h3>{ format!("Order # {}", order.id) }</h3>
                                        <p>{ format!("User ID: {}", order.user_id) }</p>
                                        <p>{ format!("Status: {}", order.status) }</p>
                                        <p>{ format!("Total: ${:.2}", order.total_amount) }</p>
                                        <button onclick={move |_| on_update.emit(order_clone.clone())}>{ "Edit" }</button>
                                        <button onclick={move |_| on_delete.emit(order_id.clone())}>{ "Delete" }</button>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    } else {
                        html! { <p>{ "Loading orders..." }</p> }
                    }
                }
            </div>
        </div>
    }
}

