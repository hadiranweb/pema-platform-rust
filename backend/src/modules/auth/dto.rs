use serde::{Deserialize, Serialize};
use validator::Validate;
use dtos::user::{LoginRequest, RegisterRequest};

// Re-export for convenience if needed, or remove if direct usage of dtos::user is preferred
pub use dtos::user::{LoginRequest, RegisterRequest};

