use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type, Encode, Decode};

// Re-export enums and structs from shared models to avoid duplication
pub use models::wallet::{WalletStatus, Wallet};
pub use models::transaction::{TransactionType, TransactionStatus, Transaction};

// Additional models for wallet service
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PurchaseFlow {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub amount: i64,
    pub status: String, // Store as string in DB
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

impl ToString for PurchaseFlowStatus {
    fn to_string(&self) -> String {
        match self {
            PurchaseFlowStatus::Pending => "pending".to_string(),
            PurchaseFlowStatus::Completed => "completed".to_string(),
            PurchaseFlowStatus::Failed => "failed".to_string(),
            PurchaseFlowStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefundRequest {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub amount: i64,
    pub reason: String,
    pub status: String, // Store as string in DB
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

impl ToString for RefundStatus {
    fn to_string(&self) -> String {
        match self {
            RefundStatus::Pending => "pending".to_string(),
            RefundStatus::Approved => "approved".to_string(),
            RefundStatus::Rejected => "rejected".to_string(),
            RefundStatus::Processed => "processed".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AdminAction {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub action_type: String, // Store as string in DB
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

impl ToString for AdminActionType {
    fn to_string(&self) -> String {
        match self {
            AdminActionType::Freeze => "freeze".to_string(),
            AdminActionType::Unfreeze => "unfreeze".to_string(),
            AdminActionType::AdjustBalance => "adjust_balance".to_string(),
            AdminActionType::ForceTransaction => "force_transaction".to_string(),
        }
    }
}

