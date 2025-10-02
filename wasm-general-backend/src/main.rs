use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use config::AppConfig;

mod error;
mod handlers;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = AppConfig::from_env().expect("Failed to load app configuration");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(
                web::scope("/api/token")
                    .route("/generate", web::post().to(handlers::token_handlers::generate_token))
                    .route("/validate", web::post().to(handlers::token_handlers::validate_token)),
            )
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}

