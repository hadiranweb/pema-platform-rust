use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::io;
use env_logger::Env;
use log::info;
use dotenv::dotenv;

mod wallet;
mod config;
mod auth_routes;
mod general_routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok(); // Load .env file

    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration
    let app_config = config::AppConfig::load();
    info!("Application configuration loaded: {:?}", app_config);

    // Database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(app_config.database.pool_size)
        .connect(&app_config.database.url())
        .await
        .expect("Failed to create Postgres connection pool");

    info!("Database connection pool established.");

    // Run database migrations (if any)
    sqlx::migrate!().run(&pool).await.expect("Failed to run database migrations");

    let server_address = format!("{}:{}", app_config.server.host, app_config.server.port);
    info!("Starting server at {}", server_address);

    HttpServer::new(move || {
        let app_config_clone = app_config.clone();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(app_config_clone.clone()))
            .configure(wallet::routes::wallet_routes)
            .configure(auth_routes::auth_config)
            .configure(general_routes::general_config)
    })
    .bind(&server_address)?
    .run()
    .await
}

