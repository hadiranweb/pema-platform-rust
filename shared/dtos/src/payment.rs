use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatePayment {
    pub order_id: Uuid,
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaymentStatus {
    pub status: String,
}

