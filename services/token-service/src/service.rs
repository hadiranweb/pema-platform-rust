use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use uuid::Uuid;
use config::AppConfig;
use crate::error::ServiceError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateTokenRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

pub fn generate_jwt_token(user_id: Uuid, config: &AppConfig) -> Result<String, ServiceError> {
    let now = Utc::now();
    let expiration = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: expiration.timestamp() as usize,
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_bytes());

    encode(&header, &claims, &encoding_key)
          .map_err(|e| ServiceError::InternalServerError(format!("Failed to encode token: {}", e).to_string()))
}

pub fn validate_jwt_token(token: &str, config: &AppConfig) -> Result<Claims, ServiceError> {
    let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
    let validation = Validation::default();

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| ServiceError::Unauthorized(format!("Invalid token: {}", e).to_string()))
}

