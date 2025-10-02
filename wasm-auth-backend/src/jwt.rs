use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use wasm_bindgen::prelude::*;

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
    let now = Utc::now();
    let expires_at = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: expires_at.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|e| JsValue::from_str(&format!("Failed to generate token: {}", e)))
}

#[wasm_bindgen]
pub fn validate_token(token: String) -> Result<String, JsValue> {
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET), &validation)
        .map(|data| data.claims.sub)
        .map_err(|e| JsValue::from_str(&format!("Failed to validate token: {}", e)))
}

