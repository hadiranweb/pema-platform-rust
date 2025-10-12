use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluginHookType {
    OnOrderCreated,
    CalculateDiscount,
    OnProductViewed,
    OnUserRegistered,
    OnPaymentProcessed,
    // Add other hook types as needed
}

impl fmt::Display for PluginHookType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscountRequest {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub original_price: f64,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscountResponse {
    pub discounted_price: f64,
    pub discount_amount: f64,
    pub reason: String,
}

pub trait PluginInterface {
    fn get_metadata() -> PluginMetadata;
    // Define common plugin functions here
    // fn on_order_created(order: Order) -> Result<(), String>;
}

