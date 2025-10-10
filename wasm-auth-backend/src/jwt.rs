use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey, Header, Token};
use sha2::Sha256;
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use std::collections::BTreeMap;

// Our claims struct, it needs to derive `Serialize` and `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

const JWT_SECRET: &[u8] = b"supersecretjwtkey"; // TODO: Load from config

#[wasm_bindgen]
pub fn generate_token(user_id: String) -> Result<String, JsValue> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET)
        .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

    let now = Utc::now();
    let expires_at = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: expires_at.timestamp() as usize,
    };

    let mut claims_map = BTreeMap::new();
    claims_map.insert("sub".to_string(), claims.sub);
    claims_map.insert("exp".to_string(), claims.exp.to_string());
    claims_map.insert("iat".to_string(), claims.iat.to_string());

    let header = Header::default();
    let token_str = Token::new(header, claims_map).sign_with_key(&key)
        .map_err(|e| JsValue::from_str(&format!("Failed to generate token: {}", e)))?;

    Ok(token_str.as_str().to_string())
}

#[wasm_bindgen]
pub fn validate_token(token: String) -> Result<String, JsValue> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET)
        .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

    let token: Token<Header, BTreeMap<String, String>, _> = token.verify_with_key(&key)
        .map_err(|e| JsValue::from_str(&format!("Failed to validate token: {}", e)))?;

    let claims = token.claims();
    claims.get("sub")
        .map(|s| s.to_string())
        .ok_or_else(|| JsValue::from_str("Token does not contain 'sub' claim"))
}

