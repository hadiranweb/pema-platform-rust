use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod jwt;

#[wasm_bindgen]
pub fn generate_auth_token(user_id: String) -> Result<String, JsValue> {
    jwt::generate_token(user_id)
}

#[wasm_bindgen]
pub fn validate_auth_token(token: String) -> Result<String, JsValue> {
    jwt::validate_token(token)
}

