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
                web::scope("/api/users")
                    .route("/health", web::get().to(handlers::health_handler::health_check))
                    .route("", web::post().to(handlers::user_handlers::create_user))
                    .route("", web::get().to(handlers::user_handlers::get_users))
                    .route("/{user_id}", web::get().to(handlers::user_handlers::get_user_by_id))
                    .route("/{user_id}", web::put().to(handlers::user_handlers::update_user))
                    .route("/{user_id}", web::delete().to(handlers::user_handlers::delete_user)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

