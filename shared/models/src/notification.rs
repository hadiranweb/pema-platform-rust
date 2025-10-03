use serde::{Deserialize, Serialize};

use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub message: String,
    pub notification_type: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNotification {
    pub user_id: Uuid,
    pub message: String,
    pub notification_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateNotification {
    pub message: Option<String>,
    pub notification_type: Option<String>,
    pub is_read: Option<bool>,
}

