use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FmtResult};
use wasm_bindgen::JsValue;

#[derive(Debug, Serialize, Clone)]
pub enum ServiceError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    DuplicateEntry(String),
    WasmError(String),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl From<ServiceError> for JsValue {
    fn from(error: ServiceError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

// This `From<String>` implementation will catch any string-based errors
// that might be passed from the host or other parts of the WASM module.
// It tries to categorize them into known ServiceError variants.
impl From<String> for ServiceError {
    fn from(err: String) -> Self {
        if err.contains("not found") {
            ServiceError::NotFound(err)
        } else if err.contains("duplicate") || err.contains("unique constraint") {
            ServiceError::DuplicateEntry(err)
        } else if err.contains("bad request") {
            ServiceError::BadRequest(err)
        } else {
            ServiceError::InternalServerError(err)
        }
    }
}

// Convert `std::env::VarError` to `ServiceError`
impl From<std::env::VarError> for ServiceError {
    fn from(err: std::env::VarError) -> Self {
        ServiceError::InternalServerError(format!("Configuration error: {}", err))
    }
}

// Convert `serde_json::Error` to `ServiceError`
impl From<serde_json::Error> for ServiceError {
    fn from(err: serde_json::Error) -> Self {
        ServiceError::InternalServerError(format!("JSON serialization/deserialization error: {}", err))
    }
}

// Convert `JsValue` to `ServiceError` for errors coming from JavaScript
impl From<JsValue> for ServiceError {
    fn from(err: JsValue) -> Self {
        ServiceError::WasmError(err.as_string().unwrap_or_else(|| "Unknown WASM error".to_string()))
    }
}

