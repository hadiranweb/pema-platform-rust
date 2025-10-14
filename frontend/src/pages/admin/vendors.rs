
use yew::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::components::forms::vendor_form::VendorForm;
use crate::services::admin_service::{AdminService, Vendor, CreateVendor, AdminVendorUpdateDto};

#[function_component(AdminVendorsPage)]
pub fn admin_vendors_page() -> Html {
    let vendors_state = use_state(|| None::<Vec<Vendor>>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);
    let show_add_form = use_state(|| false);
    let editing_vendor = use_state(|| None::<Vendor>);

    let fetch_vendors = { 
        let vendors_state = vendors_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        Callback::from(move |_| {
            let vendors_state = vendors_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_state.set(true);
                match AdminService::get_all_vendors().await {
                    Ok(vendors) => {
                        vendors_state.set(Some(vendors));
                        error_state.set(None);
                    },
                    Err(e) => {
                        log!("Failed to fetch vendors:", e.to_string());
                        error_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    use_effect_with_deps(move |fetch_vendors| {
        fetch_vendors.emit(());
        || ()
    }, fetch_vendors.clone());

    let on_add_vendor_click = { 
        let show_add_form = show_add_form.clone();
        let editing_vendor = editing_vendor.clone();
        Callback::from(move |_| {
            show_add_form.set(true);
            editing_vendor.set(None);
        })
    };

    let on_edit_vendor_click = { 
        let show_add_form = show_add_form.clone();
        let editing_vendor = editing_vendor.clone();
        Callback::from(move |vendor: Vendor| {
            show_add_form.set(true);
            editing_vendor.set(Some(vendor));
        })
    };

    let on_delete_vendor_click = { 
        let fetch_vendors = fetch_vendors.clone();
        Callback::from(move |vendor_id: String| {
            let fetch_vendors = fetch_vendors.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match AdminService::delete_vendor(vendor_id).await {
                    Ok(_) => {
                        log!("Vendor deleted successfully");
                        fetch_vendors.emit(());
                    },
                    Err(e) => {
                        log!("Failed to delete vendor:", e.to_string());
                    }
                }
            });
        })
    };

    let on_form_submit = { 
        let show_add_form = show_add_form.clone();
        let fetch_vendors = fetch_vendors.clone();
        Callback::from(move |_| {
            show_add_form.set(false);
            fetch_vendors.emit(());
        })
    };

    let on_form_cancel = { 
        let show_add_form = show_add_form.clone();
        Callback::from(move |_| {
            show_add_form.set(false);
        })
    };

    html! {
        <div class="admin-vendors-page">
            <h1>{ "Manage Vendors" }</h1>
            <button onclick={on_add_vendor_click}>{ "Add New Vendor" }</button>

            { if *show_add_form {
                html! {
                    <VendorForm 
                        vendor={(*editing_vendor).clone()}
                        on_submit={on_form_submit.clone()}
                        on_cancel={on_form_cancel.clone()}
                    />
                }
            } else { html! {} } }

            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            
            <div class="vendor-list">
                { 
                    if let Some(vendors) = &*vendors_state {
                        if vendors.is_empty() {
                            html! { <p>{ "No vendors found." }</p> }
                        } else {
                            vendors.iter().map(|vendor| {
                                let vendor_id = vendor.id.to_string();
                                let vendor_clone = vendor.clone();
                                let on_edit = on_edit_vendor_click.clone();
                                let on_delete = on_delete_vendor_click.clone();
                                html! {
                                    <div key={vendor_id.clone()} class="vendor-card">
                                        <h3>{ &vendor.name }</h3>
                                        <p>{ &vendor.description }</p>
                                        <button onclick={move |_| on_edit.emit(vendor_clone.clone())}>{ "Edit" }</button>
                                        <button onclick={move |_| on_delete.emit(vendor_id.clone())}>{ "Delete" }</button>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    } else {
                        html! { <p>{ "Loading vendors..." }</p> }
                    }
                }
            </div>
        </div>
    }
}

