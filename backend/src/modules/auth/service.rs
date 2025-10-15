
use actix_web::web;
use sqlx::PgPool;
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};

use models::user::{User, UserLogin, UserRegister};
use crate::modules::auth::repository;
use shared_config::Config;
use crate::core::plugins::manager::PluginManager;
use plugin_sdk::interface::PluginHookType;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::error::ServiceError;
use crate::services::otp;

pub struct AuthService;

impl AuthService {
pub async fn register_user(pool: &PgPool, user_register: UserRegister, config: &Config, plugin_manager: Arc<PluginManager>) -> Result<User, ServiceError> {
        let password_hash = Self::hash_password(&user_register.password).map_err(|e| ServiceError::BadRequest(e))?;
        let user = repository::create_user(pool, user_register.clone(), password_hash).await.map_err(ServiceError::DatabaseError)?;

        // Execute OnUserRegistered plugin hook
        let _ = plugin_manager.execute_hook::<String, ()>(PluginHookType::OnUserRegistered, user.id.to_string()).await.map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

        Ok(user)


    }

    pub async fn login_user(pool: &PgPool, user_login: UserLogin, config: &Config, otp_code: Option<String>) -> Result<(String, User), ServiceError> {
        let user = repository::find_user_by_email(pool, &user_login.email).await.map_err(ServiceError::DatabaseError)?;
        Self::verify_password(&user_login.password, &user.password_hash).map_err(|_| ServiceError::Unauthorized)?;

        if let Some(code) = otp_code {
            let is_otp_valid = otp::verify_otp(pool, user.id, &code).await.map_err(|e| ServiceError::InternalServerError(format!("OTP verification failed: {}", e.to_string())))?;
            if !is_otp_valid {
                return Err(ServiceError::Unauthorized);
            }
        }

        let token = Self::generate_jwt_token(&user, config).map_err(|e| ServiceError::InternalServerError(e.to_string()))?;
        Ok((token, user))
    }

    fn hash_password(password: &str) -> Result<String, ServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2.hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
    }

    fn verify_password(password: &str, password_hash: &str) -> Result<(), ServiceError> {
        let argon2 = Argon2::default();
        let parsed_hash = argon2::PasswordHash::new(password_hash)
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;
        argon2.verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| ServiceError::Unauthorized)
    }

    fn generate_jwt_token(user: &User, config: &Config) -> Result<String, ServiceError> {
        let claims = crate::modules::auth::middleware::Claims {
            sub: user.id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };
        encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret.as_bytes()))
            .map_err(|e| ServiceError::InternalServerError(e.to_string()))
    }
}

