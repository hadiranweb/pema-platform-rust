use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "wasm_general_backend"])]
    fn get_products() -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "wasm_general_backend"])]
    fn get_orders() -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "wasm_general_backend"])]
    fn get_inventory() -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "wasm_general_backend"])]
    fn get_vendors() -> JsValue;
}

pub struct ApiService;

impl ApiService {
    pub async fn fetch_products() -> Result<JsValue, String> {
        // Call the WASM general backend
        let result = get_products();
        Ok(result)
    }
    
    pub async fn fetch_orders() -> Result<JsValue, String> {
        let result = get_orders();
        Ok(result)
    }
    
    pub async fn fetch_inventory() -> Result<JsValue, String> {
        let result = get_inventory();
        Ok(result)
    }
    
    pub async fn fetch_vendors() -> Result<JsValue, String> {
        let result = get_vendors();
        Ok(result)
    }
}
