use models::order::Order;
use models::pagination::PaginatedResponse;
use super::api::{ApiService, ApiResult};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<String>,
    pub user_id: Option<Uuid>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
}

impl Default for OrderQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            status: None,
            user_id: None,
            from_date: None,
            to_date: None,
            min_amount: None,
            max_amount: None,
        }
    }
}

impl OrderQuery {
    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(format!("page={}", page));
        }
        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(status) = &self.status {
            params.push(format!("status={}", urlencoding::encode(status)));
        }
        if let Some(user_id) = self.user_id {
            params.push(format!("user_id={}", user_id));
        }
        if let Some(from_date) = self.from_date {
            params.push(format!("from_date={}", from_date.to_rfc3339()));
        }
        if let Some(to_date) = self.to_date {
            params.push(format!("to_date={}", to_date.to_rfc3339()));
        }
        if let Some(min_amount) = self.min_amount {
            params.push(format!("min_amount={}", min_amount));
        }
        if let Some(max_amount) = self.max_amount {
            params.push(format!("max_amount={}", max_amount));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrder {
    pub user_id: Uuid,
    pub items: Vec<OrderItem>,
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderStatus {
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStats {
    pub total_orders: u32,
    pub pending_orders: u32,
    pub processing_orders: u32,
    pub shipped_orders: u32,
    pub delivered_orders: u32,
    pub cancelled_orders: u32,
    pub total_revenue: f64,
    pub average_order_value: f64,
}

pub struct OrderService {
    api: ApiService,
}

impl OrderService {
    pub fn new(api: ApiService) -> Self {
        Self { api }
    }

    /// Get all orders with optional filtering and pagination
    pub async fn get_orders(&self, query: Option<OrderQuery>) -> ApiResult<PaginatedResponse<Order>> {
        let query_string = query.unwrap_or_default().to_query_string();
        let endpoint = format!("orders{}", query_string);
        self.api.get(&endpoint).await
    }

    /// Get a single order by ID
    pub async fn get_order(&self, id: Uuid) -> ApiResult<Order> {
        let endpoint = format!("orders/{}", id);
        self.api.get(&endpoint).await
    }

    /// Create a new order
    pub async fn create_order(&self, order: CreateOrder) -> ApiResult<Order> {
        self.api.post("orders", Some(order)).await
    }

    /// Update order status
    pub async fn update_order_status(&self, id: Uuid, status_update: UpdateOrderStatus) -> ApiResult<Order> {
        let endpoint = format!("orders/{}/status", id);
        self.api.put(&endpoint, Some(status_update)).await
    }

    /// Cancel an order
    pub async fn cancel_order(&self, id: Uuid, reason: Option<String>) -> ApiResult<Order> {
        let endpoint = format!("orders/{}/cancel", id);
        let body = reason.map(|r| serde_json::json!({ "reason": r }));
        self.api.put(&endpoint, body).await
    }

    /// Get orders by user
    pub async fn get_user_orders(&self, user_id: Uuid, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<Order>> {
        let query = OrderQuery {
            user_id: Some(user_id),
            page,
            limit,
            ..Default::default()
        };
        self.get_orders(Some(query)).await
    }

    /// Get orders by status
    pub async fn get_orders_by_status(&self, status: &str, page: Option<u32>, limit: Option<u32>) -> ApiResult<PaginatedResponse<Order>> {
        let query = OrderQuery {
            status: Some(status.to_string()),
            page,
            limit,
            ..Default::default()
        };
        self.get_orders(Some(query)).await
    }

    /// Get order statistics
    pub async fn get_order_stats(&self, from_date: Option<DateTime<Utc>>, to_date: Option<DateTime<Utc>>) -> ApiResult<OrderStats> {
        let mut params = Vec::new();
        if let Some(from) = from_date {
            params.push(format!("from_date={}", from.to_rfc3339()));
        }
        if let Some(to) = to_date {
            params.push(format!("to_date={}", to.to_rfc3339()));
        }
        
        let query_string = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        
        let endpoint = format!("orders/stats{}", query_string);
        self.api.get(&endpoint).await
    }

    /// Get recent orders
    pub async fn get_recent_orders(&self, limit: Option<u32>) -> ApiResult<Vec<Order>> {
        let limit = limit.unwrap_or(10);
        let endpoint = format!("orders/recent?limit={}", limit);
        self.api.get(&endpoint).await
    }

    /// Export orders to CSV
    pub async fn export_orders(&self, query: Option<OrderQuery>) -> ApiResult<String> {
        let query_string = query.unwrap_or_default().to_query_string();
        let endpoint = format!("orders/export{}", query_string);
        self.api.get(&endpoint).await
    }

    /// Get order items for a specific order
    pub async fn get_order_items(&self, order_id: Uuid) -> ApiResult<Vec<OrderItem>> {
        let endpoint = format!("orders/{}/items", order_id);
        self.api.get(&endpoint).await
    }

    /// Add tracking information to an order
    pub async fn add_tracking(&self, order_id: Uuid, tracking_number: String, carrier: String) -> ApiResult<Order> {
        let endpoint = format!("orders/{}/tracking", order_id);
        let body = serde_json::json!({
            "tracking_number": tracking_number,
            "carrier": carrier
        });
        self.api.put(&endpoint, Some(body)).await
    }

    /// Process payment for an order
    pub async fn process_payment(&self, order_id: Uuid, payment_method: String, payment_details: serde_json::Value) -> ApiResult<Order> {
        let endpoint = format!("orders/{}/payment", order_id);
        let body = serde_json::json!({
            "payment_method": payment_method,
            "payment_details": payment_details
        });
        self.api.post(&endpoint, Some(body)).await
    }
}

impl Default for OrderService {
    fn default() -> Self {
        Self::new(ApiService::default())
    }
}
