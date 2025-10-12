use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShippingDto {
    pub order_id: Uuid,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateShippingStatusDto {
    pub status: String,
}

