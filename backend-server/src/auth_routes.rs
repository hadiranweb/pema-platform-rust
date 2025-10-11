use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use wasm_auth_backend::{generate_auth_token, validate_auth_token};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenValidationRequest {
    pub token: String,
}

pub async fn auth_login(req: web::Json<AuthRequest>) -> impl Responder {
    match generate_auth_token(req.user_id.clone()) {
        Ok(token) => {
            HttpResponse::Ok().json(serde_json::json!({ "token": token }))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to generate token: {:?}", e.as_string().unwrap_or_default())),
    }
}

pub async fn auth_validate(req: web::Json<TokenValidationRequest>) -> impl Responder {
    match validate_auth_token(req.token.clone()) {
        Ok(user_id) => {
            HttpResponse::Ok().json(serde_json::json!({ "user_id": user_id }))
        },
        Err(e) => HttpResponse::Unauthorized().body(format!("Failed to validate token: {:?}", e)),
    }
}

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth")
        .route("/login", web::post().to(auth_login))
        .route("/validate", web::post().to(auth_validate))
    );
}

