use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::shared::plugin_sdk::interface::{PluginMetadata, PluginHookType};
use crate::core::plugins::sandbox::WasmPluginSandbox;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::PgPool;

pub struct PluginManager {
    active_plugins: RwLock<HashMap<Uuid, WasmPluginSandbox>>,
    // You might also have a registry for PluginMetadata
    // plugin_metadata: RwLock<HashMap<Uuid, PluginMetadata>>,
    db_pool: Arc<Mutex<PgPool>>, // Shared DB pool for host functions
}

impl PluginManager {
    pub fn new(db_pool: Arc<Mutex<PgPool>>) -> Self {
        Self {
            active_plugins: RwLock::new(HashMap::new()),
            db_pool,
        }
    }

    // This would typically load plugin WASM bytes from a DB field or a storage service
    pub async fn load_plugin(&self, plugin_id: Uuid, wasm_bytes: Vec<u8>) -> Result<(), anyhow::Error> {
        let sandbox = WasmPluginSandbox::new(&wasm_bytes, self.db_pool.clone()).await?;
        self.active_plugins.write().await.insert(plugin_id, sandbox);
        tracing::info!("Plugin {} loaded successfully.", plugin_id);
        Ok(())
    }

    pub async fn unload_plugin(&self, plugin_id: Uuid) -> Result<(), anyhow::Error> {
        self.active_plugins.write().await.remove(&plugin_id)
            .ok_or_else(|| anyhow::anyhow!("Plugin {} not found to unload", plugin_id))?;
        tracing::info!("Plugin {} unloaded successfully.", plugin_id);
        Ok(())
    }

    // This method allows the backend to trigger a hook in all active plugins
    pub async fn execute_hook<Args, Results>(
        &self,
        hook_name: &str, // e.g., "on_order_created", "calculate_discount"
        args: Args,
    ) -> Result<Vec<Results>, anyhow::Error>
    where
        Args: wasmtime::IntoWasm + Clone + Send + \'static,
        Results: wasmtime::FromWasm + Send + \'static,
    {
        let mut results = Vec::new();
        let active_plugins_guard = self.active_plugins.read().await;

        for (_id, sandbox) in active_plugins_guard.iter() {
            // Need to make a mutable copy of the sandbox/store for each call
            // Or structure WasmPluginSandbox to be cloneable/arc-able if performance is critical
            // For now, let\'s assume `call_plugin_function` handles internal mutability
            if let Ok(result) = sandbox.call_plugin_function(hook_name, args.clone()).await {
                results.push(result);
            }
        }
        Ok(results)
    }
}

