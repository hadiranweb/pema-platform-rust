use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateNotification {
    pub user_id: Uuid,
    pub message: String,
    pub notification_type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateNotification {
    pub message: Option<String>,
    pub notification_type: Option<String>,
    pub is_read: Option<bool>,
}

