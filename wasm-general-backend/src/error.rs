use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Serialize)]
pub enum ServiceError {
    InternalServerError(String),
    BadRequest(String),
    Unauthorized(String),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(msg) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .json(serde_json::json!({ "message": msg })),
            ServiceError::BadRequest(msg) => HttpResponse::build(StatusCode::BAD_REQUEST)
                .json(serde_json::json!({ "message": msg })),
            ServiceError::Unauthorized(msg) => HttpResponse::build(StatusCode::UNAUTHORIZED)
                .json(serde_json::json!({ "message": msg })),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for ServiceError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        log::error!("JWT error: {:?}", err);
        ServiceError::Unauthorized("Invalid or expired token".to_string())
    }
}

impl From<std::env::VarError> for ServiceError {
    fn from(err: std::env::VarError) -> Self {
        log::error!("Environment variable error: {:?}", err);
        ServiceError::InternalServerError("Configuration error".to_string())
    }
}

