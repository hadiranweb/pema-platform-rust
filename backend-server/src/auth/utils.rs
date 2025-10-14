use jwt::{SignWithKey, VerifyWithKey};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use shared_config::config::AppConfig;
use crate::wallet::errors::WalletError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_jwt(user_id: String, role: String, config: &AppConfig) -> Result<String, WalletError> {
    let secret = config.security.jwt_secret.as_bytes();
    let now = Utc::now();
    let expires_at = now + Duration::seconds(config.security.session_timeout as i64);
    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: expires_at.timestamp() as usize,
    };
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret)
        .map_err(|e| WalletError::InternalError(format!("Failed to create key: {}", e)))?;
    let token = claims
        .sign_with_key(&key)
        .map_err(|e| WalletError::InternalError(format!("Failed to generate token: {}", e)))?;
    Ok(token)
}

pub fn validate_jwt(token: &str, config: &AppConfig) -> Result<Claims, WalletError> {
    let secret = config.security.jwt_secret.as_bytes();
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret)
        .map_err(|e| WalletError::InternalError(format!("Failed to create key: {}", e)))?;
    let claims: Claims = token
        .verify_with_key(&key)
        .map_err(|e| WalletError::UnauthorizedAdminAction(format!("Failed to validate token: {}", e)))?;
    Ok(claims)
}

