
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::services::order_service::{OrderService, Order};

#[derive(Clone, PartialEq, Properties)]
pub struct OrderDetailProps {
    pub id: String,
}

#[function_component(OrderDetailPage)]
pub fn order_detail_page(props: &ProductDetailProps) -> Html {
    let order_state = use_state(|| None::<Order>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);

    let order_id = props.id.clone();

    { // Fetch order details on component mount or when order_id changes
        let order_state = order_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        use_effect_with_deps(move |order_id| {
            let order_id = order_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match OrderService::get_order_by_id(order_id).await {
                    Ok(order) => {
                        order_state.set(Some(order));
                        loading_state.set(false);
                    },
                    Err(e) => {
                        log!("Failed to fetch order details:", e.to_string());
                        error_state.set(Some(e.to_string()));
                        loading_state.set(false);
                    }
                }
            });
            || ()
        }, order_id);
    }

    html! {
        <div class="order-detail-page">
            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            { 
                if let Some(order) = &*order_state {
                    html! {
                        <div class="order-details">
                            <h1>{ format!("Order # {}", order.id) }</h1>
                            <p>{ format!("Status: {}", order.status) }</p>
                            <p>{ format!("Total Amount: ${:.2}", order.total_amount) }</p>
                            <p>{ format!("User ID: {}", order.user_id) }</p>
                            <p>{ format!("Created At: {}", order.created_at) }</p>
                            <p>{ format!("Updated At: {}", order.updated_at) }</p>
                        </div>
                    }
                } else if !*loading_state && error_state.is_none() {
                    html! { <p>{ "Order not found." }</p> }
                } else { html! {} }
            }
        </div>
    }
}

