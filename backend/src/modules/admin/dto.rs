use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUserUpdateDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminProductUpdateDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub category: Option<String>,
    pub vendor_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminOrderUpdateDto {
    pub status: Option<String>,
    pub total_amount: Option<f64>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminVendorUpdateDto {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminPageUpdateDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub slug: Option<String>,
    pub is_published: Option<bool>,
}

