use yew::prelude::*;
use yew_router::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::components::forms::product_form::ProductForm;
use crate::services::product_service::{ProductService, Product, CreateProduct, UpdateProduct};

#[function_component(VendorProductsPage)]
pub fn vendor_products_page() -> Html {
    let products_state = use_state(|| None::<Vec<Product>>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);
    let show_add_form = use_state(|| false);
    let editing_product = use_state(|| None::<Product>);

    let fetch_products = { 
        let products_state = products_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        Callback::from(move |_| {
            let products_state = products_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_state.set(true);
                match ProductService::get_vendor_products().await {
                    Ok(products) => {
                        products_state.set(Some(products));
                        error_state.set(None);
                    },
                    Err(e) => {
                        log!("Failed to fetch vendor products:", e.to_string());
                        error_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    use_effect_with_deps(move |fetch_products| {
        fetch_products.emit(());
        || ()
    }, fetch_products.clone());

    let on_add_product_click = { 
        let show_add_form = show_add_form.clone();
        let editing_product = editing_product.clone();
        Callback::from(move |_| {
            show_add_form.set(true);
            editing_product.set(None);
        })
    };

    let on_edit_product_click = { 
        let show_add_form = show_add_form.clone();
        let editing_product = editing_product.clone();
        Callback::from(move |product: Product| {
            show_add_form.set(true);
            editing_product.set(Some(product));
        })
    };

    let on_delete_product_click = { 
        let fetch_products = fetch_products.clone();
        Callback::from(move |product_id: String| {
            let fetch_products = fetch_products.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match ProductService::delete_product(product_id).await {
                    Ok(_) => {
                        log!("Product deleted successfully");
                        fetch_products.emit(());
                    },
                    Err(e) => {
                        log!("Failed to delete product:", e.to_string());
                    }
                }
            });
        })
    };

    let on_form_submit = { 
        let show_add_form = show_add_form.clone();
        let fetch_products = fetch_products.clone();
        Callback::from(move |_| {
            show_add_form.set(false);
            fetch_products.emit(());
        })
    };

    let on_form_cancel = { 
        let show_add_form = show_add_form.clone();
        Callback::from(move |_| {
            show_add_form.set(false);
        })
    };

    html! {
        <div class="vendor-products-page">
            <h1>{ "Your Products" }</h1>
            <button onclick={on_add_product_click}>{ "Add New Product" }</button>

            { if *show_add_form {
                html! {
                    <ProductForm 
                        product={(*editing_product).clone()}
                        on_submit={on_form_submit.clone()}
                        on_cancel={on_form_cancel.clone()}
                    />
                }
            } else { html! {} } }

            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            
            <div class="product-list">
                { 
                    if let Some(products) = &*products_state {
                        if products.is_empty() {
                            html! { <p>{ "You have no products yet." }</p> }
                        } else {
                            products.iter().map(|product| {
                                let product_id = product.id.to_string();
                                let product_clone = product.clone();
                                let on_edit = on_edit_product_click.clone();
                                let on_delete = on_delete_product_click.clone();
                                html! {
                                    <div key={product_id.clone()} class="product-card">
                                        <h3>{ &product.name }</h3>
                                        <p>{ &product.description }</p>
                                        <p>{ format!("Price: ${:.2}", product.price) }</p>
                                        <p>{ format!("Stock: {}", product.stock) }</p>
                                        <button onclick={move |_| on_edit.emit(product_clone.clone())}>{ "Edit" }</button>
                                        <button onclick={move |_| on_delete.emit(product_id.clone())}>{ "Delete" }</button>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    } else {
                        html! { <p>{ "Loading products..." }</p> }
                    }
                }
            </div>
        </div>
    }
}
