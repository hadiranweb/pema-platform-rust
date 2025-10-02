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
                web::scope("/api/vendors")
                    .route("", web::post().to(handlers::vendor_handlers::create_vendor))
                    .route("", web::get().to(handlers::vendor_handlers::get_vendors))
                    .route("/{vendor_id}", web::get().to(handlers::vendor_handlers::get_vendor_by_id))
                    .route("/{vendor_id}", web::put().to(handlers::vendor_handlers::update_vendor))
                    .route("/{vendor_id}", web::delete().to(handlers::vendor_handlers::delete_vendor)),
            )
    })
    .bind("127.0.0.1:8087")?
    .run()
    .await
}

