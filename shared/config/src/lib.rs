use serde::Deserialize;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_address: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();
        Ok(AppConfig {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            server_address: env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
        })
    }
}

