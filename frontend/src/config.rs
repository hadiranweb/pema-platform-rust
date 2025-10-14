//! Frontend application configuration.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FrontendConfig {
    pub api_base_url: String,
}

impl FrontendConfig {
    /// Loads the frontend configuration from environment variables or defaults.
    /// For WASM, these environment variables are typically injected at compile time
    /// via `Trunk.toml` or a custom build script.
    pub fn load() -> Self {
        // In a real WASM application, you might use `std::env::var` if configured
        // via `Trunk.toml` or a similar mechanism that injects env vars.
        // For simplicity and demonstration, we'll use a placeholder or a default.
        // A more robust solution would involve fetching a config.json or using build-time constants.
        Self {
            api_base_url: option_env!("API_BASE_URL")
                .unwrap_or("/") // Default to relative path if not set
                .to_string(),
        }
    }
}

