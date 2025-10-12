use yew::prelude::*;
use yew_hooks::use_state;
use crate::components::common::Input;
use crate::components::common::Button;

#[function_component(ProductForm)]
pub fn product_form() -> Html {
    let name = use_state(|| String::new());
    let description = use_state(|| String::new());
    let price = use_state(|| String::new());

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |value| name.set(value))
    };

    let on_description_change = { 
        let description = description.clone();
        Callback::from(move |value| description.set(value))
    };

    let on_price_change = { 
        let price = price.clone();
        Callback::from(move |value| price.set(value))
    };

    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        // Handle product submission logic here
        log::info!("Product submission: Name: {}, Description: {}, Price: {}", name.as_str(), description.as_str(), price.as_str());
    });

    html! {
        <form onsubmit={on_submit}>
            <Input
                label="Product Name"
                input_type="text"
                value={name.to_string()}
                onchange={on_name_change}
                placeholder="Enter product name"
                required={true}
            />
            <Input
                label="Description"
                input_type="text"
                value={description.to_string()}
                onchange={on_description_change}
                placeholder="Enter product description"
                required={true}
            />
            <Input
                label="Price"
                input_type="number"
                value={price.to_string()}
                onchange={on_price_change}
                placeholder="Enter product price"
                required={true}
            />
            <Button class="btn-primary">{ "Submit Product" }</Button>
        </form>
    }
}

