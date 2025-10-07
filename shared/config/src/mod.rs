use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub app: ApplicationConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub domain: String,
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub name: String,
    pub version: String,
    pub environment: Environment,
    pub debug: bool,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub session_timeout: u64,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Environment {
    Development,
    Production,
    Testing,
}

impl AppConfig {
    /// Load config from file or environment
    pub fn load() -> Result<Self, ConfigError> {
        // 1. Check if config file exists
        if Path::new("config.toml").exists() {
            return Self::load_from_file("config.toml");
        }
        
        // 2. Try .env file
        if Path::new(".env").exists() {
            return Self::load_from_env();
        }
        
        // 3. Return error - needs installation
        Err(ConfigError::NotInstalled)
    }
    
    /// Load from TOML file
    fn load_from_file(path: &str) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)
            .map_err(|e| ConfigError::FileReadError(e.to_string()))?;
        
        toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e.to_string()))
    }
    
    /// Load from environment variables
    fn load_from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
                port: std::env::var("SERVER_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(8080),
                domain: std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".into()),
                base_url: std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8080".into()),
            },
            database: DatabaseConfig {
                host: std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".into()),
                port: std::env::var("DB_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(5432),
                name: std::env::var("DB_NAME")?,
                user: std::env::var("DB_USER")?,
                password: std::env::var("DB_PASSWORD")?,
                pool_size: std::env::var("DB_POOL_SIZE")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(10),
            },
            app: ApplicationConfig {
                name: std::env::var("APP_NAME").unwrap_or_else(|_| "PEMA Platform".into()),
                version: env!("CARGO_PKG_VERSION").into(),
                environment: match std::env::var("ENVIRONMENT").as_deref() {
                    Ok("production") => Environment::Production,
                    Ok("testing") => Environment::Testing,
                    _ => Environment::Development,
                },
                debug: std::env::var("DEBUG")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(false),
                log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),
            },
            security: SecurityConfig {
                jwt_secret: std::env::var("JWT_SECRET")?,
                session_timeout: std::env::var("SESSION_TIMEOUT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(3600),
                cors_origins: std::env::var("CORS_ORIGINS")
                    .unwrap_or_else(|_| "*".into())
                    .split(",")
                    .map(String::from)
                    .collect(),
            },
        })
    }
    
    /// Save config to file
    pub fn save(&self, path: &str) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::SerializeError(e.to_string()))?;
        
        fs::write(path, content)
            .map_err(|e| ConfigError::FileWriteError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Check if installation is needed
    pub fn is_installed() -> bool {
        Path::new("config.toml").exists() || Path::new(".env").exists()
    }
}

#[derive(Debug)]
pub enum ConfigError {
    NotInstalled,
    FileReadError(String),
    FileWriteError(String),
    ParseError(String),
    SerializeError(String),
    MissingVariable(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::NotInstalled => write!(f, "Application not installed. Please run installer."),
            ConfigError::FileReadError(e) => write!(f, "Failed to read config file: {}", e),
            ConfigError::FileWriteError(e) => write!(f, "Failed to write config file: {}", e),
            ConfigError::ParseError(e) => write!(f, "Failed to parse config: {}", e),
            ConfigError::SerializeError(e) => write!(f, "Failed to serialize config: {}", e),
            ConfigError::MissingVariable(v) => write!(f, "Missing environment variable: {}", v),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::env::VarError> for ConfigError {
    fn from(err: std::env::VarError) -> Self {
        ConfigError::MissingVariable(err.to_string())
    }
}

