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
                web::scope("/api/orders")
                    .route("", web::post().to(handlers::order_handlers::create_order))
                    .route("", web::get().to(handlers::order_handlers::get_orders))
                    .route("/{order_id}", web::get().to(handlers::order_handlers::get_order_by_id))
                    .route("/{order_id}", web::put().to(handlers::order_handlers::update_order))
                    .route("/{order_id}", web::delete().to(handlers::order_handlers::delete_order)),
            )
    })
    .bind("127.0.0.1:8084")?
    .run()
    .await
}

