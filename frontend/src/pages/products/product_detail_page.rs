
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::services::product_service::{ProductService, Product};

#[derive(Clone, PartialEq, Properties)]
pub struct ProductDetailProps {
    pub id: String,
}

#[function_component(ProductDetailPage)]
pub fn product_detail_page(props: &ProductDetailProps) -> Html {
    let product_state = use_state(|| None::<Product>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);

    let product_id = props.id.clone();

    { // Fetch product details on component mount or when product_id changes
        let product_state = product_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        use_effect_with_deps(move |product_id| {
            let product_id = Uuid::parse_str(&product_id).expect("Invalid product ID");
            wasm_bindgen_futures::spawn_local(async move {
                match ProductService::get_product_by_id(product_id).await {
                    Ok(product) => {
                        product_state.set(Some(product));
                        loading_state.set(false);
                    },
                    Err(e) => {
                        log!("Failed to fetch product details:", e.to_string());
                        error_state.set(Some(e.to_string()));
                        loading_state.set(false);
                    }
                }
            });
            || ()
        }, product_id);
    }

    html! {
        <div class="product-detail-page">
            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            { 
                if let Some(product) = &*product_state {
                    html! {
                        <div class="product-details">
                            <h1>{ &product.name }</h1>
                            <p>{ &product.description }</p>
                            <p>{ format!("Price: ${:.2}", product.price) }</p>
                            <p>{ format!("Stock: {}", product.stock) }</p>

                            <p>{ format!("Vendor ID: {}", product.vendor_id) }</p>
                        </div>
                    }
                } else if !*loading_state && error_state.is_none() {
                    html! { <p>{ "Product not found." }</p> }
                } else { html! {} }
            }
        </div>
    }
}

