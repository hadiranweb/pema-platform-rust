use serde::{Deserialize, Serialize};

use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub total_amount: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrder {
    pub user_id: Uuid,
    pub total_amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateOrder {
    pub status: Option<String>,
    pub total_amount: Option<f64>,
}

