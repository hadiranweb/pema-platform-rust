use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateInventoryItem {
    pub product_id: Uuid,
    pub quantity: i32,
    pub location: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateInventoryItem {
    pub quantity: Option<i32>,
    pub location: Option<String>,
}

