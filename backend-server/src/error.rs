use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum PemaError {
    DatabaseError(String),
    ValidationError(String),
    NotFound(String),
    InternalError(String),
}

impl fmt::Display for PemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PemaError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            PemaError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            PemaError::NotFound(msg) => write!(f, "Not found: {}", msg),
            PemaError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl ResponseError for PemaError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PemaError::DatabaseError(_) => HttpResponse::InternalServerError().json("Database error"),
            PemaError::ValidationError(msg) => HttpResponse::BadRequest().json(msg),
            PemaError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            PemaError::InternalError(_) => HttpResponse::InternalServerError().json("Internal server error"),
        }
    }
}