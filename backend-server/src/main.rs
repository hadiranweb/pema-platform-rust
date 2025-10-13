use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use std::{io, sync::Arc};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;

// Modules
mod wallet;
mod auth;
mod auth_routes;
mod general_routes;
mod db_pool_impl;
mod core;

// Shared crates
use shared_config::config::AppConfig;

// Core systems
use core::plugins::{PluginManager, PluginConfig};
use core::events::EventBus;
use core::tenant::TenantManager;

// A global static variable to hold the PgPool, accessible by WASM host functions
// This is a common pattern for WASM host functions to access shared resources.


#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pema_backend_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let app_config = AppConfig::load();
    info!("Application configuration loaded: {:?}", app_config);

    // Database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(app_config.database.pool_size)
        .connect(&app_config.database.url)
        .await
        .expect("Failed to create Postgres connection pool");

    info!("Database connection pool established");

    // Run database migrations
    sqlx::migrate!().run(&pool).await
        .expect("Failed to run database migrations");

    // Initialize core systems
    let event_bus = Arc::new(EventBus::new());
    
    let plugin_config = PluginConfig {
        max_memory_mb: 32,
        max_execution_time_ms: 200,
        max_plugins_per_tenant: 100,
        plugin_storage_path: "./plugins".to_string(),
        enable_hot_reload: true,
    };
    
    let plugin_manager = Arc::new(
        PluginManager::new(plugin_config, event_bus.clone())
            .expect("Failed to initialize plugin manager")
    );

    let tenant_manager = Arc::new(TenantManager::new());

    info!("Core systems initialized");

    let server_address = format!("{}:{}", app_config.server.host, app_config.server.port);
    info!("Starting PEMA Platform server at {}", server_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .app_data(web::Data::new(plugin_manager.clone()))
            .app_data(web::Data::new(tenant_manager.clone()))
            .app_data(web::Data::new(event_bus.clone()))
            .configure(wallet::routes::wallet_routes)
            .configure(auth_routes::auth_config)
            .configure(general_routes::general_config)
            .configure(configure_plugin_routes)
    })
    .bind(&server_address)?
    .run()
    .await
}

/// Configure plugin management routes
fn configure_plugin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/plugins")
            .route("/upload", web::post().to(upload_plugin))
            .route("/list", web::get().to(list_plugins))
            .route("/{plugin_id}", web::delete().to(unload_plugin))
            .route("/{plugin_id}/execute", web::post().to(execute_plugin))
    );
}

/// Upload and load a new plugin
async fn upload_plugin(
    plugin_manager: web::Data<Arc<PluginManager>>,
    // TODO: Add proper request handling for plugin upload
) -> actix_web::Result<impl actix_web::Responder> {
    // TODO: Implement plugin upload logic
    Ok(web::Json(serde_json::json!({
        "status": "success",
        "message": "Plugin upload endpoint - TODO: implement"
    })))
}

/// List plugins for tenant
async fn list_plugins(
    plugin_manager: web::Data<Arc<PluginManager>>,
    // TODO: Add tenant context extraction
) -> actix_web::Result<impl actix_web::Responder> {
    // TODO: Implement plugin listing logic
    Ok(web::Json(serde_json::json!({
        "status": "success",
        "plugins": []
    })))
}

/// Unload a plugin
async fn unload_plugin(
    plugin_manager: web::Data<Arc<PluginManager>>,
    path: web::Path<String>,
) -> actix_web::Result<impl actix_web::Responder> {
    let plugin_id = path.into_inner();
    // TODO: Implement plugin unloading logic
    Ok(web::Json(serde_json::json!({
        "status": "success",
        "message": format!("Plugin {} unload endpoint - TODO: implement", plugin_id)
    })))
}

/// Execute a plugin hook
async fn execute_plugin(
    plugin_manager: web::Data<Arc<PluginManager>>,
    path: web::Path<String>,
) -> actix_web::Result<impl actix_web::Responder> {
    let plugin_id = path.into_inner();
    // TODO: Implement plugin execution logic
    Ok(web::Json(serde_json::json!({
        "status": "success",
        "message": format!("Plugin {} execution endpoint - TODO: implement", plugin_id)
    })))
}

