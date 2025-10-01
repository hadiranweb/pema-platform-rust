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
                web::scope("/api/inventory")
                    .route("/health", web::get().to(handlers::health_handler::health_check))
                    .route("", web::post().to(handlers::inventory_handlers::create_inventory_item))
                    .route("", web::get().to(handlers::inventory_handlers::get_inventory_items))
                    .route("/{item_id}", web::get().to(handlers::inventory_handlers::get_inventory_item_by_id))
                    .route("/{item_id}", web::put().to(handlers::inventory_handlers::update_inventory_item))
                    .route("/{item_id}", web::delete().to(handlers::inventory_handlers::delete_inventory_item)),
            )
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}

