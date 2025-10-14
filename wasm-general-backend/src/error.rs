use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use sqlx::Error as SqlxError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Serialize)]
pub enum ServiceError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    DuplicateEntry(String),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::NotFound(msg) => HttpResponse::build(StatusCode::NOT_FOUND)
                .json(serde_json::json!({ "message": msg })),
            ServiceError::InternalServerError(msg) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .json(serde_json::json!({ "message": msg })),
            ServiceError::BadRequest(msg) => HttpResponse::build(StatusCode::BAD_REQUEST)
                .json(serde_json::json!({ "message": msg })),
            ServiceError::DuplicateEntry(msg) => HttpResponse::build(StatusCode::CONFLICT)
                .json(serde_json::json!({ "message": msg })),
        }
    }
}

impl From<SqlxError> for ServiceError {
    fn from(err: SqlxError) -> Self {
        log::error!("SQLx error: {:?}", err);
        match err {
            SqlxError::RowNotFound => ServiceError::NotFound("Record not found".to_string()),
            SqlxError::Database(db_err) if db_err.is_unique_violation() => {
                ServiceError::DuplicateEntry("A record with this unique identifier already exists.".to_string())
            }
            _ => ServiceError::InternalServerError("Database error occurred".to_string()),
        }
    }
}

impl From<std::env::VarError> for ServiceError {
    fn from(err: std::env::VarError) -> Self {
        log::error!("Environment variable error: {:?}", err);
        ServiceError::InternalServerError("Configuration error".to_string())
    }
}

