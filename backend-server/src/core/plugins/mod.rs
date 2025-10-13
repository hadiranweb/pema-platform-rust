use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasmtime::{Engine, Module, Store, Instance, Linker, Config};
use anyhow::Result;

pub mod manager;
pub mod sandbox;
pub mod loader;
pub mod registry;

pub use manager::PluginManager;
pub use sandbox::WasmPluginSandbox;
pub use loader::PluginLoader;
pub use registry::PluginRegistry;

/// Plugin execution context
#[derive(Debug, Clone)]
pub struct PluginContext {
    pub tenant_id: String,
    pub user_id: Option<String>,
    pub request_id: String,
    pub permissions: Vec<String>,
}

/// Plugin hook types
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PluginHook {
    OnOrderCreated,
    OnOrderUpdated,
    OnPaymentProcessed,
    OnUserRegistered,
    OnProductViewed,
    OnCartUpdated,
}

/// Plugin metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub hooks: Vec<PluginHook>,
    pub permissions: Vec<String>,
    pub tenant_id: String,
}

/// Plugin execution result
#[derive(Debug)]
pub struct PluginResult {
    pub success: bool,
    pub data: serde_json::Value,
    pub logs: Vec<String>,
    pub execution_time_ms: u64,
}

/// Host functions that plugins can call
pub trait HostFunctions {
    fn log_message(&self, level: &str, message: &str) -> Result<()>;
    fn get_user_points(&self, user_id: &str) -> Result<i32>;
    fn add_user_points(&self, user_id: &str, points: i32) -> Result<()>;
    fn emit_event(&self, event_type: &str, data: serde_json::Value) -> Result<()>;
    fn get_config(&self, key: &str) -> Result<Option<String>>;
    fn set_config(&self, key: &str, value: &str) -> Result<()>;
}

/// Plugin system configuration
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub max_memory_mb: u32,
    pub max_execution_time_ms: u64,
    pub max_plugins_per_tenant: u32,
    pub plugin_storage_path: String,
    pub enable_hot_reload: bool,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 16,
            max_execution_time_ms: 100,
            max_plugins_per_tenant: 50,
            plugin_storage_path: "./plugins".to_string(),
            enable_hot_reload: true,
        }
    }
}