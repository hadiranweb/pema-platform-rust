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
                web::scope("/api/notifications")
                    .route("", web::post().to(handlers::notification_handlers::create_notification))
                    .route("", web::get().to(handlers::notification_handlers::get_notifications))
                    .route("/{notification_id}", web::get().to(handlers::notification_handlers::get_notification_by_id))
                    .route("/{notification_id}", web::put().to(handlers::notification_handlers::update_notification))
                    .route("/{notification_id}", web::delete().to(handlers::notification_handlers::delete_notification)),
            )
    })
    .bind("127.0.0.1:8085")?
    .run()
    .await
}

