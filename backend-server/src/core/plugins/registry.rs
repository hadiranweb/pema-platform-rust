use super::*;
use std::collections::HashMap;
use wasmtime::Module;

/// Plugin registry that manages loaded plugins
pub struct PluginRegistry {
    plugins: HashMap<String, RegisteredPlugin>,
    tenant_plugins: HashMap<String, Vec<String>>, // tenant_id -> plugin_ids
    hook_plugins: HashMap<(String, PluginHook), Vec<String>>, // (tenant_id, hook) -> plugin_ids
}

#[derive(Debug)]
struct RegisteredPlugin {
    metadata: PluginMetadata,
    module: Module,
    loaded_at: chrono::DateTime<chrono::Utc>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            tenant_plugins: HashMap::new(),
            hook_plugins: HashMap::new(),
        }
    }

    /// Register a new plugin
    pub async fn register_plugin(
        &mut self,
        metadata: PluginMetadata,
        module: Module,
    ) -> Result<()> {
        let plugin_id = metadata.id.clone();
        let tenant_id = metadata.tenant_id.clone();

        // Check if plugin already exists
        if self.plugins.contains_key(&plugin_id) {
            return Err(anyhow::anyhow!("Plugin '{}' already registered", plugin_id));
        }

        // Register hooks
        for hook in &metadata.hooks {
            let key = (tenant_id.clone(), hook.clone());
            self.hook_plugins
                .entry(key)
                .or_insert_with(Vec::new)
                .push(plugin_id.clone());
        }

        // Add to tenant plugins
        self.tenant_plugins
            .entry(tenant_id.clone())
            .or_insert_with(Vec::new)
            .push(plugin_id.clone());

        // Store plugin
        let registered_plugin = RegisteredPlugin {
            metadata,
            module,
            loaded_at: chrono::Utc::now(),
        };

        self.plugins.insert(plugin_id, registered_plugin);

        Ok(())
    }

    /// Unregister a plugin
    pub async fn unregister_plugin(&mut self, tenant_id: &str, plugin_id: &str) -> Result<()> {
        // Remove from plugins
        let plugin = self.plugins.remove(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("Plugin '{}' not found", plugin_id))?;

        // Remove from tenant plugins
        if let Some(tenant_plugin_list) = self.tenant_plugins.get_mut(tenant_id) {
            tenant_plugin_list.retain(|id| id != plugin_id);
            if tenant_plugin_list.is_empty() {
                self.tenant_plugins.remove(tenant_id);
            }
        }

        // Remove from hook plugins
        for hook in &plugin.metadata.hooks {
            let key = (tenant_id.to_string(), hook.clone());
            if let Some(hook_plugin_list) = self.hook_plugins.get_mut(&key) {
                hook_plugin_list.retain(|id| id != plugin_id);
                if hook_plugin_list.is_empty() {
                    self.hook_plugins.remove(&key);
                }
            }
        }

        Ok(())
    }

    /// Get plugins that handle a specific hook for a tenant
    pub async fn get_plugins_for_hook(
        &self,
        tenant_id: &str,
        hook: &PluginHook,
    ) -> Result<Vec<String>> {
        let key = (tenant_id.to_string(), hook.clone());
        Ok(self.hook_plugins.get(&key).cloned().unwrap_or_default())
    }

    /// List all plugins for a tenant
    pub async fn list_plugins(&self, tenant_id: &str) -> Result<Vec<PluginMetadata>> {
        let plugin_ids = self.tenant_plugins.get(tenant_id).cloned().unwrap_or_default();
        
        let mut plugins = Vec::new();
        for plugin_id in plugin_ids {
            if let Some(plugin) = self.plugins.get(&plugin_id) {
                plugins.push(plugin.metadata.clone());
            }
        }

        Ok(plugins)
    }

    /// Get plugin by ID
    pub async fn get_plugin(&self, plugin_id: &str) -> Result<Option<&RegisteredPlugin>> {
        Ok(self.plugins.get(plugin_id))
    }

    /// Get plugin module for execution
    pub async fn get_plugin_module(&self, plugin_id: &str) -> Result<Option<&Module>> {
        Ok(self.plugins.get(plugin_id).map(|p| &p.module))
    }

    /// Get plugin metadata
    pub async fn get_plugin_metadata(&self, plugin_id: &str) -> Result<Option<&PluginMetadata>> {
        Ok(self.plugins.get(plugin_id).map(|p| &p.metadata))
    }

    /// Check if tenant has permission to load plugin
    pub async fn check_plugin_permission(
        &self,
        tenant_id: &str,
        plugin_id: &str,
        required_permission: &str,
    ) -> Result<bool> {
        if let Some(plugin) = self.plugins.get(plugin_id) {
            // Check if plugin belongs to tenant
            if plugin.metadata.tenant_id != tenant_id {
                return Ok(false);
            }

            // Check if plugin has required permission
            Ok(plugin.metadata.permissions.contains(&required_permission.to_string()))
        } else {
            Ok(false)
        }
    }

    /// Get plugin statistics
    pub async fn get_plugin_stats(&self, tenant_id: &str) -> Result<PluginStats> {
        let plugin_ids = self.tenant_plugins.get(tenant_id).cloned().unwrap_or_default();
        
        let mut stats = PluginStats {
            total_plugins: plugin_ids.len(),
            plugins_by_hook: HashMap::new(),
            oldest_plugin: None,
            newest_plugin: None,
        };

        let mut oldest_time = None;
        let mut newest_time = None;

        for plugin_id in plugin_ids {
            if let Some(plugin) = self.plugins.get(&plugin_id) {
                // Update oldest/newest
                if oldest_time.is_none() || plugin.loaded_at < oldest_time.unwrap() {
                    oldest_time = Some(plugin.loaded_at);
                    stats.oldest_plugin = Some(plugin.metadata.clone());
                }
                if newest_time.is_none() || plugin.loaded_at > newest_time.unwrap() {
                    newest_time = Some(plugin.loaded_at);
                    stats.newest_plugin = Some(plugin.metadata.clone());
                }

                // Count hooks
                for hook in &plugin.metadata.hooks {
                    *stats.plugins_by_hook.entry(hook.clone()).or_insert(0) += 1;
                }
            }
        }

        Ok(stats)
    }
}

/// Plugin statistics
#[derive(Debug)]
pub struct PluginStats {
    pub total_plugins: usize,
    pub plugins_by_hook: HashMap<PluginHook, usize>,
    pub oldest_plugin: Option<PluginMetadata>,
    pub newest_plugin: Option<PluginMetadata>,
}