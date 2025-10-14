use serde::{Deserialize, Serialize};

use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category: String,
    pub vendor_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
}

