use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub session_timeout: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub security: SecurityConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        // For simplicity, loading from environment variables or default values
        // In a real application, you might use a config file (e.g., TOML, YAML)
        Self {
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://localhost/pema".to_string()),
                pool_size: std::env::var("DATABASE_POOL_SIZE").map(|s| s.parse().unwrap_or(10)).unwrap_or(10),
            },
            server: ServerConfig {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: std::env::var("SERVER_PORT").map(|s| s.parse().unwrap_or(8080)).unwrap_or(8080),
            },
            security: SecurityConfig {
                jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
                session_timeout: std::env::var("SESSION_TIMEOUT").map(|s| s.parse().unwrap_or(3600)).unwrap_or(3600),
            },
        }
    }
}

