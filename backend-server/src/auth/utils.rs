use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use uuid::Uuid;
use shared_config::config::AppConfig;
use crate::wallet::errors::WalletError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_jwt(user_id: Uuid, config: &AppConfig) -> Result<String, WalletError> {
    let now = Utc::now();
    let expires_at = now + Duration::seconds(config.security.session_timeout as i64);
    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp() as usize,
        exp: expires_at.timestamp() as usize,
    };
    
    let encoding_key = EncodingKey::from_secret(config.security.jwt_secret.as_bytes());
    let token = encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| WalletError::InternalError(format!("Failed to generate token: {}", e)))?;
    Ok(token)
}

pub fn validate_jwt(token: &str, config: &AppConfig) -> Result<Claims, WalletError> {
    let decoding_key = DecodingKey::from_secret(config.security.jwt_secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| WalletError::UnauthorizedAdminAction(format!("Failed to validate token: {}", e)))?;
    Ok(token_data.claims)
}

