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

