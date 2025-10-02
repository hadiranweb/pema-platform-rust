use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files::Files;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use tera::Tera;

mod handlers;
mod error;
mod validation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        log::warn!("DATABASE_URL not set, using default");
        "postgresql://postgres:password@localhost/pema".to_string()
    });

    // Try to connect to database, but don't fail if it's not available
    let pool_result = PgPool::connect(&database_url).await;
    let pool = match pool_result {
        Ok(pool) => {
            log::info!("Successfully connected to database");
            pool
        }
        Err(e) => {
            log::warn!("Failed to connect to database: {}. Running in offline mode.", e);
            // Create a dummy pool that will be handled gracefully in handlers
            PgPool::connect("postgresql://dummy:dummy@localhost/dummy").await.unwrap_or_else(|_| {
                // If even dummy connection fails, we'll handle this in the handlers
                panic!("Cannot create database pool")
            })
        }
    };

    // Initialize Tera template engine with correct path
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => {
            log::info!("Successfully initialized Tera template engine");
            t
        }
        Err(e) => {
            log::error!("Failed to initialize Tera: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Template engine initialization failed: {}", e),
            ));
        }
    };

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    
    log::info!("Starting PEMA Landing Page service on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .wrap(Logger::default())
            // Static files
            .service(Files::new("/static", "static").show_files_listing())
            // Main page
            .route("/", web::get().to(handlers::page_handlers::landing_page))
            // Health check
            .route("/health", web::get().to(handlers::page_handlers::health_check))
            // Authentication API
            .route("/api/auth/check", web::post().to(handlers::page_handlers::check_user))
            .route("/api/auth/verify", web::post().to(handlers::page_handlers::verify_code))
            .route("/api/auth/login", web::post().to(handlers::page_handlers::login))
            .route("/api/auth/register", web::post().to(handlers::page_handlers::register))
            .route("/api/auth/social", web::post().to(handlers::page_handlers::social_login))
            // Products API
            .route("/api/products", web::get().to(handlers::page_handlers::get_products))
    })
    .bind(&bind_address)?
    .run()
    .await
}
