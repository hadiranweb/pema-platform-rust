use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display("Internal Server Error: {}", _0)]
    InternalServerError(String),

    #[display("Bad Request: {}", _0)]
    BadRequest(String),

    #[display("Unauthorized")]
    Unauthorized,

    #[display("Forbidden")]
    Forbidden,

    #[display("Not Found: {}", _0)]
    NotFound(String),

    #[display("Database Error: {}", _0)]
    DatabaseError(String),

    #[display("Validation Error: {}", _0)]
    ValidationError(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(ref message) => HttpResponse::InternalServerError().json(message),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::Forbidden => HttpResponse::Forbidden().json("Forbidden"),
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            ServiceError::DatabaseError(ref message) => HttpResponse::InternalServerError().json(message),
            ServiceError::ValidationError(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}





