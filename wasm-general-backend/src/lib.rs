use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_name = fetchProductList)]
    fn fetch_product_list_from_host_js() -> js_sys::Promise;

    #[wasm_bindgen(js_name = fetchOrderDetails)]
    fn fetch_order_details_from_host_js(order_id: &str) -> js_sys::Promise;
}

#[wasm_bindgen]
pub async fn get_product_list() -> Result<JsValue, JsValue> {
    log("WASM: Requesting product list from host...");
    let promise = fetch_product_list_from_host_js();
    let products = JsFuture::from(promise).await?;
    log(&format!("WASM: Received products: {:?}", products));
    Ok(products)
}

#[wasm_bindgen]
pub async fn get_order_details(order_id: &str) -> Result<JsValue, JsValue> {
    log(&format!("WASM: Requesting details for order {} from host...", order_id));
    let promise = fetch_order_details_from_host_js(order_id);
    let details = JsFuture::from(promise).await?;
    log(&format!("WASM: Received order details: {:?}", details));
    Ok(details)
}



mod service;
mod error;

use sqlx::PgPool;
use uuid::Uuid;
use models::wallet::{CreateWallet, Wallet};

#[wasm_bindgen]
pub async fn create_new_wallet(pool: &PgPool, user_id: String, currency: String, initial_balance: Option<f64>) -> Result<JsValue, JsValue> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|e| JsValue::from_str(&format!("Invalid user_id UUID: {}", e)))?;

    let new_wallet = CreateWallet {
        user_id: user_uuid,
        currency,
        initial_balance,
    };

    let wallet = service::create_wallet(pool, new_wallet).await
        .map_err(|e| JsValue::from_str(&format!("Failed to create wallet: {}", e)))?;

    serde_wasm_bindgen::to_value(&wallet)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize wallet: {}", e)))
}

