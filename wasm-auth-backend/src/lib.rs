use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
// use uuid::Uuid; // Uuid is no longer directly used here

mod jwt;

#[wasm_bindgen]
pub fn generate_auth_token(user_id: String) -> Result<String, JsValue> {
    // user_id is already a String, pass it directly to jwt::generate_token
    jwt::generate_token(user_id)
}

#[wasm_bindgen]
pub fn validate_auth_token(token: String) -> Result<String, JsValue> {
    jwt::validate_token(token)
}

