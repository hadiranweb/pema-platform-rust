pub mod error;
mod db_interface;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use uuid::Uuid;
use models::wallet::CreateWalletRequest;
use error::ServiceError;
use serde::Serialize;

#[cfg(target_arch = "wasm32")]
use serde_wasm_bindgen::to_value as wasm_to_value;

#[cfg(not(target_arch = "wasm32"))]
use serde_json::to_value as json_to_value;

fn to_value<T: Serialize>(value: &T) -> Result<JsValue, JsValue> {
    #[cfg(target_arch = "wasm32")]
    { wasm_to_value(value).map_err(|e| JsValue::from(e.to_string())) }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let json_value = json_to_value(value)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize to JSON: {}", e)))?;
        // For non-WASM, we convert the serde_json::Value to a string and then to JsValue
        Ok(JsValue::from_str(&json_value.to_string()))
    }
}

use db_interface::{fetch_data, log_message};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn get_product_list() -> Result<JsValue, JsValue> {
    log_message("WASM: Requesting product list from host...".to_string());
    let products_promise = fetch_data("/api/products".to_string(), "GET".to_string(), JsValue::NULL);
    let products_js_value = JsFuture::from(products_promise).await.map_err(|e| JsValue::from(ServiceError::WasmError(format!("Failed to fetch product list: {:?}", e))))?;
    log_message(format!("WASM: Received products: {:?}", products_js_value));
    Ok(products_js_value)
}

#[wasm_bindgen]
pub async fn get_order_details(order_id: String) -> Result<JsValue, JsValue> {
    log_message(format!("WASM: Requesting details for order {} from host...", order_id));
    let order_details_promise = fetch_data(format!("/api/orders/{}", order_id), "GET".to_string(), JsValue::NULL);
    let details_js_value = JsFuture::from(order_details_promise).await.map_err(|e| JsValue::from(ServiceError::WasmError(format!("Failed to fetch order details: {:?}", e))))?;
    log_message(format!("WASM: Received order details: {:?}", details_js_value));
    Ok(details_js_value)
}

#[wasm_bindgen]
pub async fn create_new_wallet(user_id: String, currency: String, initial_balance: Option<f64>) -> Result<JsValue, JsValue> {
    log_message(format!("WASM: Attempting to create new wallet for user_id: {}, currency: {}, initial_balance: {:?}", user_id, currency, initial_balance));

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|e| JsValue::from(ServiceError::BadRequest(format!("Invalid user_id UUID: {}", e))))?;

    let create_wallet_payload = CreateWalletRequest {
        currency,
        initial_balance,
    };

    let payload_js_value = to_value(&create_wallet_payload)
        .map_err(|e| JsValue::from(ServiceError::InternalServerError(format!("Failed to serialize wallet payload: {:?}", e))))?;

    let wallet_creation_promise = fetch_data("/api/wallet".to_string(), "POST".to_string(), payload_js_value);
    let wallet_js_value = JsFuture::from(wallet_creation_promise).await.map_err(|e| JsValue::from(ServiceError::WasmError(format!("Failed to create wallet: {:?}", e))))?;

    log_message(format!("WASM: Wallet creation response: {:?}", wallet_js_value));

    Ok(wallet_js_value)
}

