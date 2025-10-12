use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateReview {
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateReview {
    pub rating: Option<i32>,
    pub comment: Option<String>,
}

