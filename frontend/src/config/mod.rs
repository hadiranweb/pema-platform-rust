use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FrontendConfig {
    pub api_base_url: String,
}

impl FrontendConfig {
    pub fn new() -> Self {
        // Default to relative path for development/same-origin deployment
        // In a production environment, this should be overridden by build-time environment variables
        // or fetched from a runtime configuration endpoint.
        let api_base_url = option_env!("PEMA_API_BASE_URL")
            .unwrap_or("")
            .to_string();

        FrontendConfig {
            api_base_url,
        }
    }
}

