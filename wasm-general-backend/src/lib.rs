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

