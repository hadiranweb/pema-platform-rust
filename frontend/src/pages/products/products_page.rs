
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::services::product_service::{ProductService, Product};
use crate::router::routes::AppRoute;

#[function_component(ProductsPage)]
pub fn products_page() -> Html {
    let products_state = use_state(|| None::<Vec<Product>>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);

    { // Fetch products on component mount
        let products_state = products_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match ProductService::get_all_products().await {
                    Ok(products) => {
                        products_state.set(Some(products));
                        loading_state.set(false);
                    },
                    Err(e) => {
                        log!("Failed to fetch products:", e.to_string());
                        error_state.set(Some(e.to_string()));
                        loading_state.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="products-page">
            <h1>{ "All Products" }</h1>
            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            <div class="product-list">
                { 
                    if let Some(products) = &*products_state {
                        products.iter().map(|product| {
                            let product_id = product.id.clone();
                            html! {
                                <div key={product.id.to_string()} class="product-card">
                                    <h3><Link<AppRoute> to={AppRoute::ProductDetail { id: product_id }}>{ &product.name }</Link<AppRoute>></h3>
                                    <p>{ &product.description }</p>
                                    <p>{ format!("Price: ${:.2}", product.price) }</p>
                                    <p>{ format!("Stock: {}", product.stock) }</p>
                                </div>
                            }
                        }).collect::<Html>()
                    } else {
                        html! { <p>{ "No products found." }</p> }
                    }
                }
            </div>
        </div>
    }
}

