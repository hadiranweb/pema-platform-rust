use actix_web::{web, HttpResponse};
use config::AppConfig;
use crate::{service, error::ServiceError};
use service::{GenerateTokenRequest, ValidateTokenRequest, TokenResponse};

pub async fn generate_token(config: web::Data<AppConfig>, req: web::Json<GenerateTokenRequest>) -> Result<HttpResponse, ServiceError> {
    let token = service::generate_jwt_token(req.user_id, config.get_ref())?;
    Ok(HttpResponse::Ok().json(TokenResponse { token }))
}

pub async fn validate_token(config: web::Data<AppConfig>, req: web::Json<ValidateTokenRequest>) -> Result<HttpResponse, ServiceError> {
    let claims = service::validate_jwt_token(&req.token, config.get_ref())?;
    Ok(HttpResponse::Ok().json(claims))
}

