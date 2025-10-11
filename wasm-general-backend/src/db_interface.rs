use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "fetchData")]
    pub fn fetch_data(url: String, method: String, body: JsValue) -> Promise;

    #[wasm_bindgen(js_name = "logMessage")]
    pub fn log_message(message: String);
}
