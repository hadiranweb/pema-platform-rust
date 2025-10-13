use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Re-export enums and structs from shared models to avoid duplication
pub use models::wallet::{WalletStatus, Wallet};
pub use models::transaction::{TransactionType, TransactionStatus, Transaction};

// Additional models for wallet service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseFlow {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub amount: i64,
    pub status: PurchaseFlowStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PurchaseFlowStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub amount: i64,
    pub reason: String,
    pub status: RefundStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefundStatus {
    Pending,
    Approved,
    Rejected,
    Processed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAction {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub action_type: AdminActionType,
    pub amount: Option<i64>,
    pub reason: String,
    pub admin_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdminActionType {
    Freeze,
    Unfreeze,
    AdjustBalance,
    ForceTransaction,
}

