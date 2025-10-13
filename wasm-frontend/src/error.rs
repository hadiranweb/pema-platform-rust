use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    TemplateError(tera::Error),
    ValidationError(String),
    AuthenticationError(String),
    ExternalServiceError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::TemplateError(err) => write!(f, "Template error: {}", err),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            AppError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(_) => {
                log::error!("Database error: {}", self);
                HttpResponse::InternalServerError().json(json!({
                    "error": "خطای داخلی سرور",
                    "message": "مشکلی در پایگاه داده رخ داده است"
                }))
            }
            AppError::TemplateError(_) => {
                log::error!("Template error: {}", self);
                HttpResponse::InternalServerError().json(json!({
                    "error": "خطای داخلی سرور",
                    "message": "مشکلی در رندر کردن صفحه رخ داده است"
                }))
            }
            AppError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "خطای اعتبارسنجی",
                    "message": msg
                }))
            }
            AppError::AuthenticationError(msg) => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "خطای احراز هویت",
                    "message": msg
                }))
            }
            AppError::ExternalServiceError(msg) => {
                log::warn!("External service error: {}", msg);
                HttpResponse::ServiceUnavailable().json(json!({
                    "error": "سرویس در دسترس نیست",
                    "message": "لطفاً بعداً تلاش کنید"
                }))
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<tera::Error> for AppError {
    fn from(err: tera::Error) -> Self {
        AppError::TemplateError(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::ExternalServiceError(format!("HTTP request failed: {}", err))
    }
}

pub type AppResult<T> = Result<T, AppError>;
