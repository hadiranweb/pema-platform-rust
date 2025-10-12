use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

// ============================================
// Core Event Type
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: Uuid,
    pub event_type: String,
    pub tenant_id: Uuid,
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub version: i32,
    pub payload: sqlx::types::Json<Value>,
    pub metadata: Option<sqlx::types::Json<Value>>,
    pub user_id: Option<Uuid>,
    pub occurred_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

impl Event {
    pub fn new(
        event_type: String,
        tenant_id: Uuid,
        aggregate_id: Uuid,
        aggregate_type: String,
        payload: Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            tenant_id,
            aggregate_id,
            aggregate_type,
            version: 1,
            payload: sqlx::types::Json(payload),
            metadata: None,
            user_id: None,
            occurred_at: Utc::now(),
            processed_at: None,
        }
    }

    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(sqlx::types::Json(metadata));
        self
    }
}

// ============================================
// Event Types (Constants)
// ============================================

pub mod events {
    // User events
    pub const USER_REGISTERED: &str = "user.registered";
    pub const USER_UPDATED: &str = "user.updated";
    pub const USER_DELETED: &str = "user.deleted";

    // Product events
    pub const PRODUCT_CREATED: &str = "product.created";
    pub const PRODUCT_UPDATED: &str = "product.updated";
    pub const PRODUCT_DELETED: &str = "product.deleted";
    pub const PRODUCT_OUT_OF_STOCK: &str = "product.out_of_stock";

    // Order events
    pub const ORDER_CREATED: &str = "order.created";
    pub const ORDER_CONFIRMED: &str = "order.confirmed";
    pub const ORDER_SHIPPED: &str = "order.shipped";
    pub const ORDER_DELIVERED: &str = "order.delivered";
    pub const ORDER_CANCELLED: &str = "order.cancelled";

    // Payment events
    pub const PAYMENT_INITIATED: &str = "payment.initiated";
    pub const PAYMENT_SUCCEEDED: &str = "payment.succeeded";
    pub const PAYMENT_FAILED: &str = "payment.failed";
    pub const PAYMENT_REFUNDED: &str = "payment.refunded";

    // Vendor events
    pub const VENDOR_REGISTERED: &str = "vendor.registered";
    pub const VENDOR_APPROVED: &str = "vendor.approved";
    pub const VENDOR_SUSPENDED: &str = "vendor.suspended";

    // Wallet events
    pub const WALLET_CREDITED: &str = "wallet.credited";
    pub const WALLET_DEBITED: &str = "wallet.debited";
}

// ============================================
// Event Processing Log
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EventProcessingLog {
    pub id: Uuid,
    pub event_id: Uuid,
    pub subscriber_name: String,
    pub status: String,
    pub error_message: Option<String>,
    pub attempts: i32,
    pub processed_at: DateTime<Utc>,
}

// ============================================
// Dead Letter Queue Entry
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeadLetterQueueEntry {
    pub id: Uuid,
    pub event_id: Uuid,
    pub subscriber_name: String,
    pub error_message: String,
    pub attempts: i32,
    pub last_attempt_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

