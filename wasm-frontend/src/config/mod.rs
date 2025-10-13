use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_base_url: String,
    pub app_name: String,
    pub version: String,
    pub environment: Environment,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Environment {
    Development,
    Production,
    Testing,
}

impl AppConfig {
    /// Load config from compile-time environment or defaults
    pub fn load() -> Self {
        Self {
            api_base_url: option_env!("API_BASE_URL")
                .unwrap_or("http://localhost:8080")
                .to_string(),
            app_name: "پلتفرم PEMA".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: match option_env!("APP_ENV") {
                Some("production") => Environment::Production,
                Some("testing") => Environment::Testing,
                _ => Environment::Development,
            },
        }
    }
    
    /// Get API endpoint
    pub fn api_url(&self, path: &str) -> String {
        format!("{}/{}", self.api_base_url.trim_end_matches("/"), path.trim_start_matches("/"))
    }
}

// Global config instance
static mut CONFIG: Option<AppConfig> = None;

pub fn init_config() -> AppConfig {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(AppConfig::load());
        }
        CONFIG.as_ref().unwrap().clone()
    }
}

pub fn get_config() -> AppConfig {
    unsafe {
        CONFIG.as_ref().expect("Config not initialized").clone()
    }
}

