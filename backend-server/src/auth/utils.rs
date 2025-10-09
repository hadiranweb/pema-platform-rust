
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::wallet::errors::WalletError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid, // Subject (user_id)
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
    pub role: String, // User role (e.g., "user", "admin")
}

impl Claims {
    pub fn new(user_id: Uuid, role: String, config: &AppConfig) -> Self {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::seconds(config.security.session_timeout as i64)).timestamp() as usize;
        Claims { sub: user_id, exp, iat, role }
    }
}

pub fn create_jwt(user_id: Uuid, role: String, config: &AppConfig) -> Result<String, WalletError> {
    let claims = Claims::new(user_id, role, config);
    let header = Header::default();
    encode(&header, &claims, &EncodingKey::from_secret(config.security.jwt_secret.as_bytes()))
        .map_err(|e| WalletError::InternalError(format!("Failed to create JWT: {}", e)))
}

pub fn validate_jwt(token: &str, config: &AppConfig) -> Result<Claims, WalletError> {
    decode::<Claims>(token, &DecodingKey::from_secret(config.security.jwt_secret.as_bytes()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| WalletError::UnauthorizedAdminAction(format!("Invalid JWT: {}", e)))
}

