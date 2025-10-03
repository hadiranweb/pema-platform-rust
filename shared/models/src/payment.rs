use serde::{Deserialize, Serialize};

use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub amount: f64,
    pub status: String,
    pub transaction_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatePayment {
    pub order_id: Uuid,
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaymentStatus {
    pub status: String,
}

