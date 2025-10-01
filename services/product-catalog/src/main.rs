use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod error;
mod handlers;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/products")
                    .route("/health", web::get().to(handlers::health_handler::health_check))
                    .route("", web::post().to(handlers::product_handlers::create_product))
                    .route("", web::get().to(handlers::product_handlers::get_products))
                    .route("/{product_id}", web::get().to(handlers::product_handlers::get_product_by_id))
                    .route("/{product_id}", web::put().to(handlers::product_handlers::update_product))
                    .route("/{product_id}", web::delete().to(handlers::product_handlers::delete_product)),
            )
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

