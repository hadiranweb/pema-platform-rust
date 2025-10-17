use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use std::collections::BTreeMap;
use std::cell::RefCell;

// Enhanced claims struct with all required fields
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub exp: usize,         // Expiration time
    pub iat: usize,         // Issued at
    pub nbf: usize,         // Not before
    pub iss: String,        // Issuer
    pub aud: String,        // Audience
    pub user_id: String,    // User ID
    pub email: Option<String>, // Email
    pub phone: Option<String>, // Phone
    pub role: String,       // User role
    pub permissions: Vec<String>, // Permissions
    pub session_id: String, // Session ID
    pub device_id: Option<String>, // Device ID
}

// JWT Configuration
#[derive(Debug, Clone)]
pub struct JwtConfig {
    secret: Vec<u8>,
    expiration_hours: i64,
    issuer: String,
    audience: String,
}

impl JwtConfig {
    pub fn new() -> Self {
        Self {
            secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "pema_platform_secure_jwt_secret_key_2024".to_string())
                .into_bytes(),
            expiration_hours: 24,
            issuer: "PEMA Platform".to_string(),
            audience: "PEMA Users".to_string(),
        }
    }

    pub fn get_secret(&self) -> &[u8] {
        &self.secret
    }

    pub fn get_expiration_hours(&self) -> i64 {
        self.expiration_hours
    }

    pub fn get_issuer(&self) -> &str {
        &self.issuer
    }

    pub fn get_audience(&self) -> &str {
        &self.audience
    }
}

thread_local! {
    static JWT_CONFIG: RefCell<JwtConfig> = RefCell::new(JwtConfig::new());
}

#[wasm_bindgen]
pub fn generate_token(user_id: String) -> Result<String, JsValue> {
    JWT_CONFIG.with(|config_cell| {
        let config = config_cell.borrow();
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        let now = Utc::now();
        let expires_at = now + Duration::hours(config.get_expiration_hours());
        let session_id = uuid::Uuid::new_v4().to_string();

        let mut claims_map = BTreeMap::new();
        claims_map.insert("sub".to_string(), user_id.clone());
        claims_map.insert("exp".to_string(), (expires_at.timestamp() as usize).to_string());
        claims_map.insert("iat".to_string(), (now.timestamp() as usize).to_string());
        claims_map.insert("nbf".to_string(), (now.timestamp() as usize).to_string());
        claims_map.insert("iss".to_string(), config.get_issuer().to_string());
        claims_map.insert("aud".to_string(), config.get_audience().to_string());
        claims_map.insert("user_id".to_string(), user_id);
        claims_map.insert("role".to_string(), "user".to_string());
        claims_map.insert("session_id".to_string(), session_id);

        let token_str = claims_map.sign_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to sign token: {}", e)))?;

        Ok(token_str)
    })
}

#[wasm_bindgen]
pub fn generate_token_with_details(
    user_id: String,
    email: Option<String>,
    phone: Option<String>,
    role: String,
    permissions: Vec<String>,
    device_id: Option<String>,
) -> Result<String, JsValue> {
    JWT_CONFIG.with(|config_cell| {
        let config = config_cell.borrow();
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        let now = Utc::now();
        let expires_at = now + Duration::hours(config.get_expiration_hours());
        let session_id = uuid::Uuid::new_v4().to_string();

        let mut claims_map = BTreeMap::new();
        claims_map.insert("sub".to_string(), user_id.clone());
        claims_map.insert("exp".to_string(), (expires_at.timestamp() as usize).to_string());
        claims_map.insert("iat".to_string(), (now.timestamp() as usize).to_string());
        claims_map.insert("nbf".to_string(), (now.timestamp() as usize).to_string());
        claims_map.insert("iss".to_string(), config.get_issuer().to_string());
        claims_map.insert("aud".to_string(), config.get_audience().to_string());
        claims_map.insert("user_id".to_string(), user_id);
        claims_map.insert("role".to_string(), role);
        claims_map.insert("session_id".to_string(), session_id);

        if let Some(email) = email {
            claims_map.insert("email".to_string(), email);
        }
        if let Some(phone) = phone {
            claims_map.insert("phone".to_string(), phone);
        }
        if let Some(device_id) = device_id {
            claims_map.insert("device_id".to_string(), device_id);
        }

        // Add permissions as comma-separated string
        if !permissions.is_empty() {
            claims_map.insert("permissions".to_string(), permissions.join(","));
        }

        let token_str = claims_map.sign_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to sign token: {}", e)))?;

        Ok(token_str)
    })
}

#[wasm_bindgen]
pub fn verify_token(token: String) -> Result<String, JsValue> {
    JWT_CONFIG.with(|config_cell| {
        let config = config_cell.borrow();
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        let claims: BTreeMap<String, String> = token.verify_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to verify token: {}", e)))?;

        // Check expiration
        if let Some(exp_str) = claims.get("exp") {
            let exp: usize = exp_str.parse()
                .map_err(|_| JsValue::from_str("Invalid expiration time"))?;
            let now = Utc::now().timestamp() as usize;
            if now >= exp {
                return Err(JsValue::from_str("Token has expired"));
            }
        }

        // Check not before
        if let Some(nbf_str) = claims.get("nbf") {
            let nbf: usize = nbf_str.parse()
                .map_err(|_| JsValue::from_str("Invalid not before time"))?;
            let now = Utc::now().timestamp() as usize;
            if now < nbf {
                return Err(JsValue::from_str("Token not yet valid"));
            }
        }

        // Return claims as JSON string
        let claims_json = serde_json::to_string(&claims)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize claims: {}", e)))?;

        Ok(claims_json)
    })
}

#[wasm_bindgen]
pub fn refresh_token(token: String) -> Result<String, JsValue> {
    JWT_CONFIG.with(|config_cell| {
        let config = config_cell.borrow();
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        // Verify the existing token
        let claims: BTreeMap<String, String> = token.verify_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to verify token: {}", e)))?;

        // Extract user information
        let user_id = claims.get("user_id")
            .ok_or_else(|| JsValue::from_str("Missing user_id in token"))?
            .clone();

        let role = claims.get("role").unwrap_or(&"user".to_string()).clone();
        let email = claims.get("email").cloned();
        let phone = claims.get("phone").cloned();
        let device_id = claims.get("device_id").cloned();
        
        let permissions = claims.get("permissions")
            .map(|p| p.split(',').map(|s| s.to_string()).collect())
            .unwrap_or_else(|| vec!["read".to_string()]);

        // Generate new token
        generate_token_with_details(user_id, email, phone, role, permissions, device_id)
    })
}

#[wasm_bindgen]
pub fn get_token_claims(token: String) -> Result<String, JsValue> {
    JWT_CONFIG.with(|config_cell| {
        let config = config_cell.borrow();
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        let claims: BTreeMap<String, String> = token.verify_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to verify token: {}", e)))?;

        let claims_json = serde_json::to_string(&claims)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize claims: {}", e)))?;

        Ok(claims_json)
    })
}

#[wasm_bindgen]
pub fn is_token_expired(token: String) -> Result<bool, JsValue> {
    JWT_CONFIG.with(|config_cell| {
        let config = config_cell.borrow();
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        let claims: BTreeMap<String, String> = token.verify_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to verify token: {}", e)))?;

        if let Some(exp_str) = claims.get("exp") {
            let exp: usize = exp_str.parse()
                .map_err(|_| JsValue::from_str("Invalid expiration time"))?;
            let now = Utc::now().timestamp() as usize;
            Ok(now >= exp)
        } else {
            Ok(true) // If no expiration, consider it expired
        }
    })
}

#[wasm_bindgen]
pub fn validate_token_permissions(token: String, required_permission: String) -> Result<bool, JsValue> {
    JWT_CONFIG.with(|config_cell| {
        let config = config_cell.borrow();
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        let claims: BTreeMap<String, String> = token.verify_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to verify token: {}", e)))?;

        // Check if token is expired
        if let Some(exp_str) = claims.get("exp") {
            let exp: usize = exp_str.parse()
                .map_err(|_| JsValue::from_str("Invalid expiration time"))?;
            let now = Utc::now().timestamp() as usize;
            if now >= exp {
                return Ok(false);
            }
        }

        // Check permissions
        if let Some(permissions_str) = claims.get("permissions") {
            let permissions: Vec<&str> = permissions_str.split(',').collect();
            Ok(permissions.contains(&required_permission.as_str()) || permissions.contains(&"admin"))
        } else {
            Ok(false)
        }
    })
}