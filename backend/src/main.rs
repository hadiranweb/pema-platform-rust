
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use tracing_subscriber;
use tokio::sync::Mutex;

mod core;
mod config;
mod middleware;
mod modules;
mod error;
mod utils;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to create pool");

    // Initialize plugin manager
    let plugin_manager = Arc::new(core::plugins::manager::PluginManager::new(Arc::new(Mutex::new(pool.clone()))));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(plugin_manager.clone()))
            // Register routes from modules
            .configure(modules::auth::routes::init_routes)
            .configure(modules::products::routes::init_routes)
            .configure(modules::orders::routes::init_routes)
            .configure(modules::reviews::routes::init_routes)
            .configure(modules::shipping::routes::init_routes)
            .configure(modules::vendors::routes::init_routes)
            .configure(modules::admin::routes::init_routes)
            .configure(modules::pages::routes::init_routes)
            .configure(modules::wallet::routes::init_routes)
            // Add other module routes here
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

