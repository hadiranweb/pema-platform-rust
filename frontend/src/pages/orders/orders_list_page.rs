use yew::prelude::*;
use yew_router::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::services::order_service::{OrderService, Order};
use crate::state::auth::AuthStore;
use yewdux::prelude::use_store;
use crate::router::routes::AppRoute;

#[function_component(OrdersListPage)]
pub fn orders_list_page() -> Html {
    let orders_state = use_state(|| None::<Vec<Order>>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);

    { // Fetch orders on component mount
        let orders_state = orders_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let (auth_state, _) = use_store::<AuthStore>();
                if let Some(token) = auth_state.token.clone() {
                    if let Some(user_id) = auth_state.user_id {
                        match OrderService::get_user_orders(&token, user_id).await {
                            Ok(orders) => {
                                orders_state.set(Some(orders));
                                loading_state.set(false);
                            },
                            Err(e) => {
                                log!("Failed to fetch orders:", e.to_string());
                                error_state.set(Some(e.to_string()));
                                loading_state.set(false);
                            }
                        }
                    } else {
                        log!("User ID not available for fetching orders.");
                        error_state.set(Some("User ID not available.".to_string()));
                        loading_state.set(false);
                    }
                } else {
                    log!("Authentication token not available for fetching orders.");
                    error_state.set(Some("Authentication required.".to_string()));
                    loading_state.set(false);
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="orders-list-page">
            <h1>{ "My Orders" }</h1>
            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            <div class="order-list">
                { 
                    if let Some(orders) = &*orders_state {
                        if orders.is_empty() {
                            html! { <p>{ "You have no orders yet." }</p> }
                        } else {
                            orders.iter().map(|order| {
                                let order_id = order.id.clone();
                                html! {
                                    <div key={order.id.to_string()} class="order-card">
                                        <h3><Link<AppRoute> to={AppRoute::OrderDetail { id: order_id }}>{ format!("Order # {}", order.id) }</Link<AppRoute>></h3>
                                        <p>{ format!("Status: {}", order.status) }</p>
                                        <p>{ format!("Total: ${:.2}", order.total_price) }</p>
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
