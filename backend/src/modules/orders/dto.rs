use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use dtos::order::{CreateOrder as DtosCreateOrder, UpdateOrder as DtosUpdateOrder};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
pub struct CreateOrder {
    #[validate(custom = "crate::utils::validators::is_uuid")]
    pub product_id: Uuid,
    #[validate(range(min = 1))]
    pub quantity: i32,
    #[validate(range(min = 0.01))]
    pub total_price: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
pub struct UpdateOrder {
    pub status: Option<String>,
    pub quantity: Option<i32>,
    pub total_price: Option<f64>,
}

// If the backend needs to convert from its own validated DTOs to the shared DTOs, 
// these `From` implementations would be useful. Otherwise, the backend can directly use `dtos::order::CreateOrder`
// if the validation is moved to the shared DTOs.
// For now, keeping the validation in the backend's DTOs and providing conversion.
impl From<CreateOrder> for DtosCreateOrder {
    fn from(dto: CreateOrder) -> Self {
        Self {
            product_id: dto.product_id,
            quantity: dto.quantity,
            total_price: dto.total_price,
        }
    }
}

impl From<UpdateOrder> for DtosUpdateOrder {
    fn from(dto: UpdateOrder) -> Self {
        Self {
            status: dto.status,
            quantity: dto.quantity,
            total_price: dto.total_price,
        }
    }
}

