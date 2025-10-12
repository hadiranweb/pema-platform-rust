
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::models::order::{CreateOrder, Order, UpdateOrder};
use crate::modules::orders::repository;
use crate::core::plugins::manager::PluginManager;
use pema_plugin_sdk::interface::PluginHookType;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct OrderService;

impl OrderService {
    pub async fn create_order(pool: &PgPool, mut create_order: CreateOrder, user_id: Uuid, plugin_manager: Arc<PluginManager>) -> Result<Order, String> {
        // Prepare DiscountRequest
        let discount_request = pema_plugin_sdk::interface::DiscountRequest {
            user_id,
            product_id: create_order.product_id,
            original_price: create_order.total_price,
            quantity: create_order.quantity,
        };

        // Execute CalculateDiscount plugin hook
        let discount_responses = plugin_manager.execute_hook::<pema_plugin_sdk::interface::DiscountRequest, pema_plugin_sdk::interface::DiscountResponse>(PluginHookType::CalculateDiscount, discount_request).await;

        // Apply the first valid discount, if any
        if let Ok(responses) = discount_responses {
            if let Some(discount_response) = responses.into_iter().next() {
                create_order.total_price = discount_response.discounted_price;
                // Optionally, store discount_response.reason in the order for auditing
            }
        }

        let order = repository::create_order(pool, create_order, user_id).await?;

        // Execute OnOrderCreated plugin hook
        let _ = plugin_manager.execute_hook::<String, ()>(PluginHookType::OnOrderCreated, order.id.to_string()).await;

        Ok(order)
    }

    pub async fn get_order_by_id(pool: &PgPool, order_id: Uuid) -> Result<Order, String> {
        repository::find_order_by_id(pool, order_id).await
    }

    pub async fn get_all_orders(pool: &PgPool) -> Result<Vec<Order>, String> {
        repository::find_all_orders(pool).await
    }

    pub async fn get_user_orders(pool: &PgPool, user_id: Uuid) -> Result<Vec<Order>, String> {
        repository::find_user_orders(pool, user_id).await
    }

    pub async fn update_order(pool: &PgPool, order_id: Uuid, update_order: UpdateOrder, user_id: Uuid) -> Result<Order, String> {
        // First, check if the order exists and belongs to the user
        let existing_order = repository::find_order_by_id(pool, order_id).await?;
        if existing_order.user_id != user_id {
            return Err("Order does not belong to this user".to_string());
        }
        repository::update_order(pool, order_id, update_order).await
    }

    pub async fn delete_order(pool: &PgPool, order_id: Uuid, user_id: Uuid) -> Result<(), String> {
        // First, check if the order exists and belongs to the user
        let existing_order = repository::find_order_by_id(pool, order_id).await?;
        if existing_order.user_id != user_id {
            return Err("Order does not belong to this user".to_string());
        }
        repository::delete_order(pool, order_id).await
    }
}

