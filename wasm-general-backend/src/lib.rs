use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_product_list() -> String {
    // In a real application, this would fetch data from a database or another service
    "[\"Product 1\", \"Product 2\", \"Product 3\"]".to_string()
}

#[wasm_bindgen]
pub fn get_order_details(order_id: &str) -> String {
    // In a real application, this would fetch data from a database or another service
    format!("Details for order {}", order_id)
}

