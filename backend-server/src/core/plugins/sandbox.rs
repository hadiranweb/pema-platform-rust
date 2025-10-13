use super::*;
use wasmtime::*;
use std::sync::Arc;
use tokio::sync::Mutex;

/// WASM plugin sandbox for secure execution
pub struct WasmPluginSandbox {
    plugin_id: String,
    context: PluginContext,
    config: PluginConfig,
    logs: Arc<Mutex<Vec<String>>>,
    engine: Engine,
    store: Store<HostState>,
    instance: Option<Instance>,
}

/// Host state passed to WASM instance
pub struct HostState {
    pub context: PluginContext,
    pub logs: Arc<Mutex<Vec<String>>>,
}

impl WasmPluginSandbox {
    pub fn new(
        plugin_id: String,
        context: PluginContext,
        config: PluginConfig,
    ) -> Result<Self> {
        // Configure WASM engine with security limits
        let mut wasm_config = Config::new();
        wasm_config.wasm_memory64(false);
        wasm_config.wasm_multi_memory(false);
        wasm_config.wasm_threads(false);
        wasm_config.wasm_simd(false);
        wasm_config.consume_fuel(true);
        
        let engine = Engine::new(&wasm_config)?;
        
        let logs = Arc::new(Mutex::new(Vec::new()));
        let host_state = HostState {
            context: context.clone(),
            logs: logs.clone(),
        };
        
        let mut store = Store::new(&engine, host_state);
        
        // Set fuel limit (prevents infinite loops)
        store.set_fuel(1_000_000)?; // Adjust based on needs
        
        Ok(Self {
            plugin_id,
            context,
            config,
            logs,
            engine,
            store,
            instance: None,
        })
    }

    /// Initialize the WASM instance with host functions
    pub async fn initialize(&mut self, module: &Module) -> Result<()> {
        let mut linker = Linker::new(&self.engine);
        
        // Define host functions that plugins can call
        self.define_host_functions(&mut linker)?;
        
        // Instantiate the module
        let instance = linker.instantiate(&mut self.store, module)?;
        self.instance = Some(instance);
        
        Ok(())
    }

    /// Execute a plugin hook
    pub async fn execute_hook(
        &mut self,
        hook: PluginHook,
        data: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let instance = self.instance.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Plugin not initialized"))?;

        // Get the hook function from the WASM module
        let hook_name = self.hook_to_function_name(&hook);
        let func = instance
            .get_typed_func::<(i32, i32), i32>(&mut self.store, &hook_name)
            .map_err(|_| anyhow::anyhow!("Hook function '{}' not found", hook_name))?;

        // Serialize input data to JSON string
        let input_json = serde_json::to_string(&data)?;
        
        // Allocate memory in WASM for input
        let (input_ptr, input_len) = self.allocate_string(&input_json)?;
        
        // Call the plugin function
        let result_ptr = func.call(&mut self.store, (input_ptr, input_len as i32))?;
        
        // Read result from WASM memory
        let result_json = self.read_string_from_memory(result_ptr)?;
        
        // Parse result
        let result: serde_json::Value = serde_json::from_str(&result_json)
            .unwrap_or_else(|_| serde_json::json!({ "error": "Invalid JSON response from plugin" }));
        
        Ok(result)
    }

    /// Define host functions available to plugins
    fn define_host_functions(&self, linker: &mut Linker<HostState>) -> Result<()> {
        // log_message(level_ptr: i32, level_len: i32, msg_ptr: i32, msg_len: i32)
        linker.func_wrap("env", "log_message", |mut caller: Caller<'_, HostState>, level_ptr: i32, level_len: i32, msg_ptr: i32, msg_len: i32| -> i32 {
            let memory = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -1,
            };

            let level = match read_string_from_memory(&caller, &memory, level_ptr, level_len) {
                Ok(s) => s,
                Err(_) => return -1,
            };

            let message = match read_string_from_memory(&caller, &memory, msg_ptr, msg_len) {
                Ok(s) => s,
                Err(_) => return -1,
            };

            // Add to logs
            let logs = caller.data().logs.clone();
            tokio::spawn(async move {
                let mut logs = logs.lock().await;
                logs.push(format!("[{}] {}", level, message));
            });

            0 // Success
        })?;

        // get_user_points(user_id_ptr: i32, user_id_len: i32) -> i32
        linker.func_wrap("env", "get_user_points", |mut caller: Caller<'_, HostState>, user_id_ptr: i32, user_id_len: i32| -> i32 {
            let memory = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -1,
            };

            let _user_id = match read_string_from_memory(&caller, &memory, user_id_ptr, user_id_len) {
                Ok(s) => s,
                Err(_) => return -1,
            };

            // TODO: Implement actual user points lookup
            // For now, return a dummy value
            100
        })?;

        // add_user_points(user_id_ptr: i32, user_id_len: i32, points: i32) -> i32
        linker.func_wrap("env", "add_user_points", |mut caller: Caller<'_, HostState>, user_id_ptr: i32, user_id_len: i32, points: i32| -> i32 {
            let memory = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return -1,
            };

            let _user_id = match read_string_from_memory(&caller, &memory, user_id_ptr, user_id_len) {
                Ok(s) => s,
                Err(_) => return -1,
            };

            // TODO: Implement actual user points addition
            // For now, just log the action
            let logs = caller.data().logs.clone();
            tokio::spawn(async move {
                let mut logs = logs.lock().await;
                logs.push(format!("Added {} points to user", points));
            });

            0 // Success
        })?;

        Ok(())
    }

    /// Convert hook enum to function name
    fn hook_to_function_name(&self, hook: &PluginHook) -> String {
        match hook {
            PluginHook::OnOrderCreated => "on_order_created".to_string(),
            PluginHook::OnOrderUpdated => "on_order_updated".to_string(),
            PluginHook::OnPaymentProcessed => "on_payment_processed".to_string(),
            PluginHook::OnUserRegistered => "on_user_registered".to_string(),
            PluginHook::OnProductViewed => "on_product_viewed".to_string(),
            PluginHook::OnCartUpdated => "on_cart_updated".to_string(),
        }
    }

    /// Allocate string in WASM memory
    fn allocate_string(&mut self, s: &str) -> Result<(i32, usize)> {
        let instance = self.instance.as_ref().unwrap();
        let bytes = s.as_bytes();
        
        // Get allocate function from WASM
        let allocate = instance
            .get_typed_func::<i32, i32>(&mut self.store, "allocate")
            .map_err(|_| anyhow::anyhow!("allocate function not found"))?;
        
        // Allocate memory
        let ptr = allocate.call(&mut self.store, bytes.len() as i32)?;
        
        // Write string to memory
        let memory = instance
            .get_memory(&mut self.store, "memory")
            .ok_or_else(|| anyhow::anyhow!("memory export not found"))?;
        
        memory.write(&mut self.store, ptr as usize, bytes)?;
        
        Ok((ptr, bytes.len()))
    }

    /// Read string from WASM memory
    fn read_string_from_memory(&mut self, ptr: i32) -> Result<String> {
        let instance = self.instance.as_ref().unwrap();
        let memory = instance
            .get_memory(&mut self.store, "memory")
            .ok_or_else(|| anyhow::anyhow!("memory export not found"))?;

        // First, read the length (assuming it's stored at ptr)
        let mut len_bytes = [0u8; 4];
        memory.read(&self.store, ptr as usize, &mut len_bytes)?;
        let len = u32::from_le_bytes(len_bytes) as usize;

        // Then read the actual string data
        let mut string_bytes = vec![0u8; len];
        memory.read(&self.store, (ptr + 4) as usize, &mut string_bytes)?;

        String::from_utf8(string_bytes)
            .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
    }

    /// Get accumulated logs
    pub fn get_logs(&self) -> Vec<String> {
        // This is a simplified version - in practice you'd want to handle async properly
        Vec::new() // TODO: Implement proper log retrieval
    }
}

/// Helper function to read string from WASM memory
fn read_string_from_memory(
    caller: &Caller<'_, HostState>,
    memory: &Memory,
    ptr: i32,
    len: i32,
) -> Result<String> {
    let mut bytes = vec![0u8; len as usize];
    memory.read(caller, ptr as usize, &mut bytes)
        .map_err(|e| anyhow::anyhow!("Failed to read memory: {}", e))?;
    
    String::from_utf8(bytes)
        .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
}