use pema_plugin_sdk::interface::PluginMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountRequest {
    pub user_id: String,
    pub product_id: String,
    pub original_price: f64,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountResponse {
    pub discounted_price: f64,
    pub discount_amount: f64,
    pub reason: String,
}

// TODO: Add pema_plugin macro when available
mod my_plugin {
    use super::*;

    fn get_metadata() -> PluginMetadata {
        PluginMetadata {
            id: "discount-calculator".to_string(),
            name: "Discount Calculator".to_string(),
            version: "1.0.0".to_string(),
            description: "Calculates discounts based on various rules.".to_string(),
        }
    }

    fn calculate_discount(request: DiscountRequest) -> DiscountResponse {
        let mut discounted_price = request.original_price * request.quantity as f64;
        let mut discount_amount = 0.0;
        let mut reason = "No discount applied.".to_string();

        // Example discount rule: 10% off for orders over $100
        if discounted_price > 100.0 {
            discount_amount = discounted_price * 0.10;
            discounted_price -= discount_amount;
            reason = "10% off for orders over $100.".to_string();
        }

        DiscountResponse {
            discounted_price,
            discount_amount,
            reason,
        }
    }
}

