use super::*;
use wasmtime::{Engine, Module, Config};
use std::path::Path;
use tokio::fs;

/// Plugin loader responsible for loading WASM modules
pub struct PluginLoader {
    engine: Engine,
    config: PluginConfig,
}

impl PluginLoader {
    pub fn new(config: PluginConfig) -> Result<Self> {
        // Configure WASM engine with security settings
        let mut wasm_config = Config::new();
        
        // Security settings
        wasm_config.wasm_memory64(false);
        wasm_config.wasm_multi_memory(false);
        wasm_config.wasm_threads(false);
        wasm_config.wasm_simd(false);
        wasm_config.consume_fuel(true);
        
        // Performance settings
        wasm_config.cranelift_opt_level(wasmtime::OptLevel::Speed);
        wasm_config.strategy(wasmtime::Strategy::Cranelift);
        
        let engine = Engine::new(&wasm_config)?;

        Ok(Self { engine, config })
    }

    /// Load WASM module from bytes
    pub async fn load_module(&self, wasm_bytes: &[u8]) -> Result<Module> {
        // Validate WASM module
        self.validate_wasm_module(wasm_bytes)?;
        
        // Create module
        let module = Module::new(&self.engine, wasm_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to create WASM module: {}", e))?;

        // Validate module exports
        self.validate_module_exports(&module)?;

        Ok(module)
    }

    /// Load WASM module from file
    pub async fn load_module_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Module> {
        let wasm_bytes = fs::read(path).await
            .map_err(|e| anyhow::anyhow!("Failed to read WASM file: {}", e))?;
        
        self.load_module(&wasm_bytes).await
    }

    /// Validate WASM module for security
    fn validate_wasm_module(&self, wasm_bytes: &[u8]) -> Result<()> {
        // Check file size
        if wasm_bytes.len() > (self.config.max_memory_mb as usize * 1024 * 1024) {
            return Err(anyhow::anyhow!("WASM module too large"));
        }

        // Basic WASM magic number check
        if wasm_bytes.len() < 8 || &wasm_bytes[0..4] != b"\0asm" {
            return Err(anyhow::anyhow!("Invalid WASM magic number"));
        }

        // Check WASM version
        let version = u32::from_le_bytes([
            wasm_bytes[4], wasm_bytes[5], wasm_bytes[6], wasm_bytes[7]
        ]);
        if version != 1 {
            return Err(anyhow::anyhow!("Unsupported WASM version: {}", version));
        }

        Ok(())
    }

    /// Validate that the module has required exports
    fn validate_module_exports(&self, module: &Module) -> Result<()> {
        let exports: Vec<_> = module.exports().collect();
        
        // Check for required exports
        let required_exports = vec!["memory", "allocate", "deallocate"];
        let mut found_exports = std::collections::HashSet::new();
        
        for export in &exports {
            found_exports.insert(export.name());
        }

        for required in &required_exports {
            if !found_exports.contains(required) {
                return Err(anyhow::anyhow!("Missing required export: {}", required));
            }
        }

        // Check for at least one hook function
        let hook_functions = vec![
            "on_order_created",
            "on_order_updated", 
            "on_payment_processed",
            "on_user_registered",
            "on_product_viewed",
            "on_cart_updated",
        ];

        let has_hook = hook_functions.iter().any(|hook| found_exports.contains(hook));
        if !has_hook {
            return Err(anyhow::anyhow!("Module must export at least one hook function"));
        }

        Ok(())
    }

    /// Get engine reference
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Precompile module for faster loading
    pub async fn precompile_module(&self, wasm_bytes: &[u8]) -> Result<Vec<u8>> {
        let module = self.load_module(wasm_bytes).await?;
        let compiled = module.serialize()
            .map_err(|e| anyhow::anyhow!("Failed to serialize module: {}", e))?;
        Ok(compiled)
    }

    /// Load precompiled module
    pub async fn load_precompiled_module(&self, compiled_bytes: &[u8]) -> Result<Module> {
        // Safety: We trust precompiled modules since they were compiled by us
        unsafe {
            Module::deserialize(&self.engine, compiled_bytes)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize module: {}", e))
        }
    }

    /// Scan directory for plugin files
    pub async fn scan_plugin_directory<P: AsRef<Path>>(&self, dir: P) -> Result<Vec<PluginFile>> {
        let mut plugins = Vec::new();
        let mut entries = fs::read_dir(dir).await
            .map_err(|e| anyhow::anyhow!("Failed to read plugin directory: {}", e))?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "wasm" {
                        // Look for corresponding metadata file
                        let mut metadata_path = path.clone();
                        metadata_path.set_extension("json");
                        
                        if metadata_path.exists() {
                            let metadata_content = fs::read_to_string(&metadata_path).await?;
                            let metadata: PluginMetadata = serde_json::from_str(&metadata_content)
                                .map_err(|e| anyhow::anyhow!("Invalid plugin metadata: {}", e))?;
                            
                            plugins.push(PluginFile {
                                wasm_path: path,
                                metadata_path,
                                metadata,
                            });
                        }
                    }
                }
            }
        }

        Ok(plugins)
    }
}

/// Plugin file information
#[derive(Debug)]
pub struct PluginFile {
    pub wasm_path: std::path::PathBuf,
    pub metadata_path: std::path::PathBuf,
    pub metadata: PluginMetadata,
}