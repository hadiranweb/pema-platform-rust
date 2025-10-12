use serde::{Deserialize, Serialize};

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

