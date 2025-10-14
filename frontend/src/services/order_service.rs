use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub total_price: f64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    pub quantity: Option<i32>,
    pub status: Option<String>,
}

pub struct OrderService;

impl OrderService {
    pub async fn create_order(token: &str, request: CreateOrderRequest) -> Result<Order> {
        let response = Request::post("/api/orders")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let order: Order = response.json().await?;
            Ok(order)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to create order: {}", error_text))
        }
    }

    pub async fn get_order_by_id(token: &str, order_id: Uuid) -> Result<Order> {
        let response = Request::get(&format!("/api/orders/{}", order_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            let order: Order = response.json().await?;
            Ok(order)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to fetch order: {}", error_text))
        }
    }

    pub async fn get_all_orders(token: &str) -> Result<Vec<Order>> {
        let response = Request::get("/api/orders")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            let orders: Vec<Order> = response.json().await?;
            Ok(orders)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to fetch orders: {}", error_text))
        }
    }

    pub async fn get_user_orders(token: &str, user_id: Uuid) -> Result<Vec<Order>> {
        let response = Request::get(&format!("/api/users/{}/orders", user_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            let orders: Vec<Order> = response.json().await?;
            Ok(orders)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to fetch user orders: {}", error_text))
        }
    }

    pub async fn update_order(token: &str, order_id: Uuid, request: UpdateOrderRequest) -> Result<Order> {
        let response = Request::put(&format!("/api/orders/{}", order_id))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let order: Order = response.json().await?;
            Ok(order)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to update order: {}", error_text))
        }
    }

    pub async fn delete_order(token: &str, order_id: Uuid) -> Result<()> {
        let response = Request::delete(&format!("/api/orders/{}", order_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to delete order: {}", error_text))
        }
    }
}

