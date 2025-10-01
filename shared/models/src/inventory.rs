use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct InventoryItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub location: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateInventoryItem {
    pub product_id: Uuid,
    pub quantity: i32,
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateInventoryItem {
    pub quantity: Option<i32>,
    pub location: Option<String>,
}

