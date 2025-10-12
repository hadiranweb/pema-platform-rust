use wasmtime::{Engine, Instance, Module, Store, Caller, Linker, TypedFunc};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};
use anyhow::Result;
use tracing;





use pema_plugin_sdk::interface::{PluginInterface, PluginMetadata};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PluginHostContext {
    pub wasi: WasiCtx,
    pub db_pool: Arc<Mutex<sqlx::PgPool>>,
}

pub struct WasmPluginSandbox {
    store: Store<PluginHostContext>,
    instance: Instance,
}

impl WasmPluginSandbox {
    pub async fn new(wasm_bytes: &[u8], db_pool: Arc<Mutex<sqlx::PgPool>>) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm_bytes)?;

        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdout()
            .build();
        
        let host_context = PluginHostContext {
            wasi: wasi_ctx,
            db_pool,
        };
        let mut store = Store::new(&engine, host_context);

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi)?;

        linker.func_wrap(
            "pema_host",
            "log_message", 
            |mut caller: Caller<PluginHostContext>, message_ptr: i32, message_len: i32| -> Result<()> {
                let (memory, mut store) = caller.data_and_store_mut();
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
            |mut caller: Caller<PluginHostContext>, query_ptr: i32, query_len: i32| -> Result<i32> {
                tracing::warn!("Plugin attempted to execute generic DB read query. This should be restricted to specific predefined APIs.");
                Ok(0)
            },
        )?;
        
        linker.func_wrap(
            "pema_host",
            "get_product_count_for_tenant",
            |mut caller: Caller<'_, PluginHostContext>, tenant_id_ptr: i32, tenant_id_len: i32| -> Result<i32> {
                Ok(100)
            }
        )?;

        let instance = linker.instantiate_async(&mut store, &module).await?;

        Ok(Self { store, instance })
    }

    pub async fn call_plugin_function<Args, Results>(
        &mut self,
        func_name: &str,
        args: Args,
    ) -> Result<Results>
    where
        Args: wasmtime::WasmParams,
        Results: wasmtime::WasmResults,
    {
        let func = self.instance.get_typed_func::<Args, Results>(&mut self.store, func_name)?;
        func.call_async(&mut self.store, args).await
    }
}

