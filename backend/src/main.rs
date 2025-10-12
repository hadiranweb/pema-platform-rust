use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

mod core;
mod modules;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

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
            .configure(modules::auth::routes::config)
            .configure(modules::products::routes::config)
            // Add other module routes here
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

