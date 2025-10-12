use wasmtime::{Engine, Instance, Module, Store, TypedFunc, Caller, Linker};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};
use anyhow::Result;
use crate::shared::plugin_sdk::interface::{PluginInterface, PluginMetadata};
use std::sync::Arc;
use tokio::sync::Mutex; // برای مثال دسترسی امن به دیتابیس

// Context for the WASM store, might contain DB connection, Event Bus sender, etc.
pub struct PluginHostContext {
    pub wasi: WasiCtx,
    pub db_pool: Arc<Mutex<sqlx::PgPool>>, // Shared DB pool, access via Host Functions
    // pub event_publisher: Arc<EventPublisher>,
    // ... other resources plugin might need to access
}

pub struct WasmPluginSandbox {
    store: Store<PluginHostContext>,
    instance: Instance,
}

impl WasmPluginSandbox {
    pub async fn new(wasm_bytes: &[u8], db_pool: Arc<Mutex<sqlx::PgPool>>) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm_bytes)?;

        // Configure WASI (WebAssembly System Interface) for basic I/O
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdout() // Allow plugins to print to stdout (for debugging)
            .build();
        
        let host_context = PluginHostContext {
            wasi: wasi_ctx,
            db_pool,
            // event_publisher: Arc::new(EventPublisher::new()),
        };
        let mut store = Store::new(&engine, host_context);

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi)?;

        // Link Host Functions: These are the *only* ways a plugin can interact with the outside world
        linker.func_wrap(
            "pema_host", // The module name exposed to WASM
            "log_message", 
            |mut caller: Caller<\'_, PluginHostContext>, message_ptr: i32, message_len: i32| -> Result<()> {
                // Read message from WASM memory
                let (memory, mut store) = caller.as_memory_and_store_mut();
                let message_bytes = memory.data(&mut store).get(message_ptr as usize..)
                    .and_then(|arr| arr.get(..message_len as usize))
                    .ok_or_else(|| anyhow::anyhow!("Failed to read message from WASM memory"))?;
                let message = std::str::from_utf8(message_bytes)?;
                tracing::info!("Plugin Log: {}", message);
                Ok(())
            },
        )?;

        linker.func_wrap(
            "pema_host",
            "db_execute_read_query",
            |mut caller: Caller<\'_, PluginHostContext>, query_ptr: i32, query_len: i32| -> Result<i32> {
                // IMPORTANT: Implement strict access control here.
                // Plugins should NOT be able to run arbitrary SQL.
                // Instead, they should call predefined functions like `get_product_count_for_tenant`.
                // Example (simplified):
                // let query_str = // ... read query from WASM memory ...
                // let pool = caller.data().db_pool.lock().await;
                // let result = sqlx::query(query_str).fetch_all(&*pool).await;
                // ... write result back to WASM memory ...
                // This function should return a pointer to result or an error code.
                tracing::warn!("Plugin attempted to execute generic DB read query. This should be restricted to specific predefined APIs.");
                Ok(0) // Return 0 for success, non-zero for error
            },
        )?;
        
        // Example for a controlled DB access (recommended over generic query execution)
        linker.func_wrap(
            "pema_host",
            "get_product_count_for_tenant",
            |mut caller: Caller<\'_, PluginHostContext>, tenant_id_ptr: i32, tenant_id_len: i32| -> Result<i32> {
                // ... securely read tenant_id from WASM memory ...
                // let pool = caller.data().db_pool.lock().await;
                // let count = sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE tenant_id = $1", tenant_id).fetch_one(&*pool).await?;
                // Ok(count as i32)
                Ok(100) // Dummy count
            }
        )?;


        let instance = linker.instantiate_async(&mut store, &module).await?;

        Ok(Self { store, instance })
    }

    // Call a function exported by the WASM plugin
    // This is how the backend (host) invokes plugin code (e.g., a hook)
    pub async fn call_plugin_function<Args, Results>(
        &mut self,
        func_name: &str,
        args: Args,
    ) -> Result<Results>
    where
        Args: wasmtime::IntoWasm,
        Results: wasmtime::FromWasm,
    {
        let func = self.instance.get_typed_func::<Args, Results>(&mut self.store, func_name)?;
        func.call_async(&mut self.store, args).await
    }
}

