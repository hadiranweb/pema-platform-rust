use serde::{Deserialize, Serialize};

use uuid::Uuid;

pub mod manager;

pub use manager::TenantManager;

/// Tenant information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub subdomain: Option<String>,
    pub status: TenantStatus,
    pub settings: TenantSettings,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Tenant status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TenantStatus {
    Active,
    Suspended,
    Inactive,
}

/// Tenant-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSettings {
    pub max_plugins: u32,
    pub max_users: u32,
    pub features: Vec<String>,
    pub custom_branding: Option<BrandingSettings>,
    pub database_config: DatabaseConfig,
    pub plugin_config: PluginConfig,
}

/// Branding settings for tenant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingSettings {
    pub logo_url: Option<String>,
    pub primary_color: String,
    pub secondary_color: String,
    pub font_family: Option<String>,
    pub custom_css: Option<String>,
}

/// Database configuration per tenant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub schema_name: String,
    pub max_connections: u32,
    pub connection_timeout_seconds: u64,
}

/// Plugin configuration per tenant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub max_plugins: u32,
    pub max_memory_mb: u32,
    pub max_execution_time_ms: u64,
    pub allowed_permissions: Vec<String>,
}

impl Default for TenantSettings {
    fn default() -> Self {
        Self {
            max_plugins: 10,
            max_users: 100,
            features: vec!["basic".to_string()],
            custom_branding: None,
            database_config: DatabaseConfig {
                schema_name: format!("tenant_{}", Uuid::new_v4().to_string().replace('-', "_")),
                max_connections: 10,
                connection_timeout_seconds: 30,
            },
            plugin_config: PluginConfig {
                max_plugins: 10,
                max_memory_mb: 16,
                max_execution_time_ms: 100,
                allowed_permissions: vec![
                    "read_orders".to_string(),
                    "read_users".to_string(),
                    "write_logs".to_string(),
                ],
            },
        }
    }
}

/// Tenant context for requests
#[derive(Debug, Clone)]
pub struct TenantContext {
    pub tenant: Tenant,
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
    pub request_id: String,
}

/// Tenant resolution strategy
#[derive(Debug, Clone)]
pub enum TenantResolutionStrategy {
    Domain,      // Extract from domain (e.g., tenant1.example.com)
    Subdomain,   // Extract from subdomain
    Header,      // Extract from X-Tenant-ID header
    Path,        // Extract from URL path (/tenant/{id}/...)
}

/// Tenant resolution result
#[derive(Debug)]
pub enum TenantResolution {
    Found(Tenant),
    NotFound,
    Invalid(String),
}