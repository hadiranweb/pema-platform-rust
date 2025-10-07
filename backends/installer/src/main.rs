use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;
use shared_config::{AppConfig, ConfigError, ServerConfig, DatabaseConfig, ApplicationConfig, SecurityConfig, Environment};
use std::path::Path;

#[derive(Deserialize)]
pub struct InstallData {
    server: ServerConfigData,
    database: DatabaseConfigData,
    app: ApplicationConfigData,
    security: SecurityConfigData,
}

#[derive(Deserialize)]
pub struct ServerConfigData {
    domain: String,
    base_url: String,
}

#[derive(Deserialize)]
pub struct DatabaseConfigData {
    host: String,
    port: u16,
    name: String,
    user: String,
    password: String,
}

#[derive(Deserialize)]
pub struct ApplicationConfigData {
    name: String,
    environment: String,
    debug: bool,
}

#[derive(Deserialize)]
pub struct SecurityConfigData {
    jwt_secret: String,
    session_timeout: u64,
    cors_origins: Vec<String>,
}

async fn install_handler(data: web::Json<InstallData>) -> impl Responder {
    // Check if already installed
    if AppConfig::is_installed() {
        return HttpResponse::BadRequest().json(serde_json::json!({ "message": "Application already installed." }));
    }

    let new_config = AppConfig {
        server: ServerConfig {
            host: "0.0.0.0".to_string(), // Fixed for backend server
            port: 8080, // Fixed for backend server
            domain: data.server.domain.clone(),
            base_url: data.server.base_url.clone(),
        },
        database: DatabaseConfig {
            host: data.database.host.clone(),
            port: data.database.port,
            name: data.database.name.clone(),
            user: data.database.user.clone(),
            password: data.database.password.clone(),
            pool_size: 10, // Default pool size
        },
        app: ApplicationConfig {
            name: data.app.name.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: match data.app.environment.as_str() {
                "production" => Environment::Production,
                "testing" => Environment::Testing,
                _ => Environment::Development,
            },
            debug: data.app.debug,
            log_level: "info".to_string(), // Default log level
        },
        security: SecurityConfig {
            jwt_secret: data.security.jwt_secret.clone(),
            session_timeout: data.security.session_timeout,
            cors_origins: data.security.cors_origins.clone(),
        },
    };

    match new_config.save("config.toml") {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "message": "Installation successful!" })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "message": format!("Installation failed: {}", e) })),
    }
}

async fn serve_installer() -> impl Responder {
    let html_content = include_str!("../templates/install.html");
    HttpResponse::Ok().content_type("text/html").body(html_content)
}

#[actix_web::main]
asyn fn main() -> std::io::Result<()> {
    // Check if already installed, if so, exit or redirect
    if AppConfig::is_installed() {
        println!("Application already installed. Exiting installer.");
        // In a real scenario, you might redirect to the main application or exit gracefully.
        // For now, we'll just exit.
        std::process::exit(0);
    }

    HttpServer::new(|| {
        App::new()
            .route("/install", web::post().to(install_handler))
            .route("/", web::get().to(serve_installer))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

