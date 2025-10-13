use yew::prelude::*;
use yew_hooks::use_state;
use crate::components::common::Input;
use crate::components::common::Button;

#[function_component(OrderForm)]
pub fn order_form() -> Html {
    let product_id = use_state(|| String::new());
    let quantity = use_state(|| String::new());

    let on_product_id_change = {
        let product_id = product_id.clone();
        Callback::from(move |value| product_id.set(value))
    };

    let on_quantity_change = { 
        let quantity = quantity.clone();
        Callback::from(move |value| quantity.set(value))
    };

    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        // Handle order submission logic here
        log::info!("Order submission: Product ID: {}, Quantity: {}", product_id.as_str(), quantity.as_str());
    });

    html! {
        <form onsubmit={on_submit}>
            <Input
                label="Product ID"
                input_input_type="text"
                value={product_id.to_string()}
                onchange={on_product_id_change}
                placeholder="Enter product ID"
                required={true}
            />
            <Input
                label="Quantity"
                input_input_type="number"
                value={quantity.to_string()}
                onchange={on_quantity_change}
                placeholder="Enter quantity"
                required={true}
            />
            <Button class="btn-primary">{ "Place Order" }</Button>
        </form>
    }
}

