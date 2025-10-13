
use yew::prelude::*;
use gloo_console::log;
use web_sys::HtmlInputElement;

use crate::components::common::input::Input;
use crate::components::common::button::Button;
use crate::components::common::spinner::Spinner;
use crate::services::admin_service::{AdminService, CreateVendor, AdminVendorUpdateDto, Vendor};

#[derive(Properties, PartialEq)]
pub struct VendorFormProps {
    pub vendor: Option<Vendor>,
    pub on_submit: Callback<()>,
    pub on_cancel: Callback<()>,
}

#[function_component(VendorForm)]
pub fn vendor_form(props: &VendorFormProps) -> Html {
    let name_state = use_state(|| props.vendor.as_ref().map_or(String::new(), |v| v.name.clone()));
    let description_state = use_state(|| props.vendor.as_ref().map_or(String::new(), |v| v.description.clone().unwrap_or_default()));
    let loading_state = use_state(|| false);
    let error_message_state = use_state(|| Option::<String>::None);

    let on_name_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        name_state.set(input.value());
    });

    let on_description_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        description_state.set(input.value());
    });

    let on_submit = { 
        let name_state = name_state.clone();
        let description_state = description_state.clone();
        let loading_state = loading_state.clone();
        let error_message_state = error_message_state.clone();
        let on_submit_callback = props.on_submit.clone();
        let vendor = props.vendor.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let name = (*name_state).clone();
            let description = (*description_state).clone();
            let loading_state = loading_state.clone();
            let error_message_state = error_message_state.clone();
            let on_submit_callback = on_submit_callback.clone();
            let vendor = vendor.clone();

            loading_state.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let result = if let Some(existing_vendor) = vendor {
                    let update_dto = AdminVendorUpdateDto {
                        name: Some(name),
                        description: Some(description),
                    };
                    AdminService::update_vendor(existing_vendor.id.to_string(), update_dto).await
                } else {
                    let create_vendor = CreateVendor {
                        name,
                        description: Some(description),
                    };
                    AdminService::create_vendor(create_vendor).await
                };

                match result {
                    Ok(_) => {
                        log!("Vendor saved successfully");
                        error_message_state.set(None);
                        on_submit_callback.emit(());
                    },
                    Err(e) => {
                        log!("Failed to save vendor:", e.to_string());
                        error_message_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    let on_cancel = props.on_cancel.clone();

    html! {
        <div class="vendor-form">
            <h2>{ if props.vendor.is_some() { "Edit Vendor" } else { "Add New Vendor" } }</h2>
            <form onsubmit={on_submit}>
                <Input
                    label="Name"
                    id="vendor-name"
                    input_type="text"
                    value={(*name_state).clone()}
                    on_change={on_name_change}
                    placeholder="Vendor Name"
                />
                <Input
                    label="Description"
                    id="vendor-description"
                    input_type="text"
                    value={(*description_state).clone()}
                    on_change={on_description_change}
                    placeholder="Vendor Description"
                />
                { if *loading_state { html! { <Spinner /> } } else { html! {} } }
                { if let Some(msg) = &*error_message_state { html! { <p class="error-message">{ msg }</p> } } else { html! {} } }
                <Button label={ if props.vendor.is_some() { "Save Changes" } else { "Add Vendor" } } button_input_type="submit" />
                <Button label="Cancel" button_input_type="button" onclick={on_cancel} />
            </form>
        </div>
    }
}

