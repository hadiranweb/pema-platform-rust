use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
pub struct CreateOrder {
    #[validate(custom = "crate::utils::validators::is_uuid")]
    pub product_id: Uuid,
    #[validate(range(min = 1))]
    pub quantity: i32,
    #[validate(range(min = 0.01))]
    pub total_price: f64,
}

pub use crate::shared::models::order::{Order, UpdateOrder};

