use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub total_price: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateOrder {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub total_price: f64,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateOrder {
    pub quantity: Option<i32>,
    pub total_price: Option<f64>,
    pub status: Option<String>,
}

