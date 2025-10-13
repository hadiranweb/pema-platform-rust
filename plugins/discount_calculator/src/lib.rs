use plugin_sdk::interface::{PluginMetadata, DiscountRequest, DiscountResponse};
use serde::{Deserialize, Serialize};

#[no_mangle]
pub extern "C" fn get_metadata() -> *const u8 {
    let metadata = PluginMetadata {
        id: "discount-calculator".to_string(),
        name: "Discount Calculator".to_string(),
        version: "1.0.0".to_string(),
        description: "Calculates discounts based on various rules.".to_string(),
    };
    
    let json = serde_json::to_string(&metadata).unwrap();
    let bytes = json.into_bytes();
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    ptr
}

#[no_mangle]
pub extern "C" fn calculate_discount(input_ptr: *const u8, input_len: usize) -> *const u8 {
    let input_slice = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let input_str = std::str::from_utf8(input_slice).unwrap();
    let request: DiscountRequest = serde_json::from_str(input_str).unwrap();
    
    let mut discounted_price = request.original_price * request.quantity as f64;
    let mut discount_amount = 0.0;
    let mut reason = "No discount applied.".to_string();

    // Example discount rule: 10% off for orders over $100
    if discounted_price > 100.0 {
        discount_amount = discounted_price * 0.10;
        discounted_price -= discount_amount;
        reason = "10% off for orders over $100.".to_string();
    }

    let response = DiscountResponse {
        discounted_price,
        discount_amount,
        reason,
    };
    
    let json = serde_json::to_string(&response).unwrap();
    let bytes = json.into_bytes();
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    ptr
}

