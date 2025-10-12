use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOtp {
    pub user_id: Uuid,
    pub code: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyOtp {
    pub user_id: Uuid,
    pub code: String,
}

