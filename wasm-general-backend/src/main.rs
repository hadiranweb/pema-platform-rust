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
                web::scope("/api/payments")
                    .route("/process", web::post().to(handlers::payment_handlers::process_payment))
                    .route("/{payment_id}", web::get().to(handlers::payment_handlers::get_payment_status)),
            )
    })
    .bind("127.0.0.1:8086")?
    .run()
    .await
}

