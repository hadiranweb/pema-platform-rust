use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey, Header, Token};
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
    JWT_CONFIG.with(|config| {
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        let now = Utc::now();
        let expires_at = now + Duration::hours(config.get_expiration_hours());

        let session_id = uuid::Uuid::new_v4().to_string();

        let claims = Claims {
            sub: user_id.clone(),
            iat: now.timestamp() as usize,
            exp: expires_at.timestamp() as usize,
            nbf: now.timestamp() as usize,
            iss: config.get_issuer().to_string(),
            aud: config.get_audience().to_string(),
            user_id,
            email: None,
            phone: None,
            role: "user".to_string(),
            permissions: vec!["read".to_string()],
            session_id,
            device_id: None,
        };

        let mut claims_map = BTreeMap::new();
        claims_map.insert("sub".to_string(), claims.sub);
        claims_map.insert("exp".to_string(), claims.exp.to_string());
        claims_map.insert("iat".to_string(), claims.iat.to_string());
        claims_map.insert("nbf".to_string(), claims.nbf.to_string());
        claims_map.insert("iss".to_string(), claims.iss);
        claims_map.insert("aud".to_string(), claims.aud);
        claims_map.insert("user_id".to_string(), claims.user_id);
        claims_map.insert("role".to_string(), claims.role);
        claims_map.insert("session_id".to_string(), claims.session_id);

        let header = Header::default();
        let token_str = Token::new(header, claims_map).sign_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate token: {}", e)))?;

        Ok(token_str.as_str().to_string())
    })
}

#[wasm_bindgen]
pub fn generate_enhanced_token(
    user_id: String,
    email: Option<String>,
    phone: Option<String>,
    role: String,
    permissions: Vec<String>,
    device_id: Option<String>,
) -> Result<String, JsValue> {
    JWT_CONFIG.with(|config| {
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        let now = Utc::now();
        let expires_at = now + Duration::hours(config.get_expiration_hours());

        let session_id = uuid::Uuid::new_v4().to_string();

        let claims = Claims {
            sub: user_id.clone(),
            iat: now.timestamp() as usize,
            exp: expires_at.timestamp() as usize,
            nbf: now.timestamp() as usize,
            iss: config.get_issuer().to_string(),
            aud: config.get_audience().to_string(),
            user_id,
            email,
            phone,
            role,
            permissions,
            session_id,
            device_id,
        };

        let mut claims_map = BTreeMap::new();
        claims_map.insert("sub".to_string(), claims.sub);
        claims_map.insert("exp".to_string(), claims.exp.to_string());
        claims_map.insert("iat".to_string(), claims.iat.to_string());
        claims_map.insert("nbf".to_string(), claims.nbf.to_string());
        claims_map.insert("iss".to_string(), claims.iss);
        claims_map.insert("aud".to_string(), claims.aud);
        claims_map.insert("user_id".to_string(), claims.user_id);
        claims_map.insert("role".to_string(), claims.role);
        claims_map.insert("session_id".to_string(), claims.session_id);

        if let Some(email) = claims.email {
            claims_map.insert("email".to_string(), email);
        }
        if let Some(phone) = claims.phone {
            claims_map.insert("phone".to_string(), phone);
        }
        if let Some(device_id) = claims.device_id {
            claims_map.insert("device_id".to_string(), device_id);
        }

        let header = Header::default();
        let token_str = Token::new(header, claims_map).sign_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate token: {}", e)))?;

        Ok(token_str.as_str().to_string())
    })
}

#[wasm_bindgen]
pub fn validate_token(token: String) -> Result<String, JsValue> {
    JWT_CONFIG.with(|config| {
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        let token: Token<Header, BTreeMap<String, String>, _> = token.verify_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to validate token: {}", e)))?;

        let claims = token.claims();
        
        // Validate expiration
        if let Some(exp_str) = claims.get("exp") {
            if let Ok(exp) = exp_str.parse::<i64>() {
                let now = Utc::now().timestamp();
                if now > exp {
                    return Err(JsValue::from_str("Token has expired"));
                }
            }
        }

        // Validate not before
        if let Some(nbf_str) = claims.get("nbf") {
            if let Ok(nbf) = nbf_str.parse::<i64>() {
                let now = Utc::now().timestamp();
                if now < nbf {
                    return Err(JsValue::from_str("Token is not yet valid"));
                }
            }
        }

        // Validate issuer
        if let Some(iss) = claims.get("iss") {
            if iss != config.get_issuer() {
                return Err(JsValue::from_str("Invalid token issuer"));
            }
        }

        // Validate audience
        if let Some(aud) = claims.get("aud") {
            if aud != config.get_audience() {
                return Err(JsValue::from_str("Invalid token audience"));
            }
        }

        claims.get("sub")
            .map(|s| s.to_string())
            .ok_or_else(|| JsValue::from_str("Token does not contain 'sub' claim"))
    })
}

#[wasm_bindgen]
pub fn get_token_claims(token: String) -> Result<String, JsValue> {
    JWT_CONFIG.with(|config| {
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        let token: Token<Header, BTreeMap<String, String>, _> = token.verify_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to validate token: {}", e)))?;

        let claims = token.claims();
        
        // Convert claims to JSON string
        serde_json::to_string(claims)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize claims: {}", e)))
    })
}

#[wasm_bindgen]
pub fn refresh_token(old_token: String) -> Result<String, JsValue> {
    // First validate the old token
    let user_id = validate_token(old_token.clone())?;
    
    JWT_CONFIG.with(|config| {
        let key: Hmac<Sha256> = Hmac::new_from_slice(config.get_secret())
            .map_err(|e| JsValue::from_str(&format!("Failed to create key: {}", e)))?;

        // Parse old token to get existing claims
        let old_token_parsed: Token<Header, BTreeMap<String, String>, _> = old_token.verify_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse old token: {}", e)))?;

        let old_claims = old_token_parsed.claims();

        // Create new token with extended expiration
        let now = Utc::now();
        let expires_at = now + Duration::hours(config.get_expiration_hours());

        let mut new_claims = BTreeMap::new();
        
        // Copy existing claims
        for (key, value) in old_claims.iter() {
            if key != "exp" && key != "iat" && key != "session_id" {
                new_claims.insert(key.clone(), value.clone());
            }
        }

        // Update timestamps and session
        new_claims.insert("iat".to_string(), now.timestamp().to_string());
        new_claims.insert("exp".to_string(), expires_at.timestamp().to_string());
        new_claims.insert("session_id".to_string(), uuid::Uuid::new_v4().to_string());

        let header = Header::default();
        let token_str = Token::new(header, new_claims).sign_with_key(&key)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate refreshed token: {}", e)))?;

        Ok(token_str.as_str().to_string())
    })
}

#[wasm_bindgen]
pub fn is_token_expired(token: String) -> bool {
    JWT_CONFIG.with(|config| {
        let key = match Hmac::new_from_slice(config.get_secret()) {
            Ok(k) => k,
            Err(_) => return true,
        };

        let token: Token<Header, BTreeMap<String, String>, _> = match token.verify_with_key(&key) {
            Ok(t) => t,
            Err(_) => return true,
        };

        let claims = token.claims();
        
        if let Some(exp_str) = claims.get("exp") {
            if let Ok(exp) = exp_str.parse::<i64>() {
                let now = Utc::now().timestamp();
                return now > exp;
            }
        }
        
        true // If we can't parse expiration, consider it expired
    })
}

