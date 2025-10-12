use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateOrder {
    pub product_id: Uuid,
    pub quantity: i32,
    pub total_price: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateOrder {
    pub status: Option<String>,
    pub quantity: Option<i32>,
    pub total_price: Option<f64>,
}

