use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use plugin_sdk::interface::{PluginMetadata, PluginHookType};
use crate::core::plugins::sandbox::WasmPluginSandbox;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::PgPool;
use crate::tracing;
use wasmtime::{WasmParams, WasmResults};

pub struct PluginManager {
    active_plugins: RwLock<HashMap<Uuid, WasmPluginSandbox>>,
    db_pool: Arc<Mutex<PgPool>>,
}

impl PluginManager {
    pub fn new(db_pool: Arc<Mutex<PgPool>>) -> Self {
        Self {
            active_plugins: RwLock::new(HashMap::new()),
            db_pool,
        }
    }

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

    pub async fn execute_hook<Args: serde::Serialize + Clone + Send + 'static, Results: serde::de::DeserializeOwned + Send + 'static>(
        &self,
        hook_type: PluginHookType,
        args: Args,
    ) -> Result<Vec<Results>, anyhow::Error>
    {
        let mut results = Vec::new();
        let mut active_plugins_guard = self.active_plugins.write().await;

                for (_id, sandbox) in active_plugins_guard.iter_mut() {
            if let Ok(result) = sandbox.call_plugin_function(&hook_type.to_string(), args.clone()).await {
                results.push(result);
            }
        }
        Ok(results)
    }
}

