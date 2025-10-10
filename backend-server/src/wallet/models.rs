use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use std::str::FromStr;

// --- Enums ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum WalletStatus {
    Active,
    Inactive,
    Suspended,
}

impl FromStr for WalletStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(WalletStatus::Active),
            "inactive" => Ok(WalletStatus::Inactive),
            "suspended" => Ok(WalletStatus::Suspended),
            _ => Err(format!("Invalid wallet status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Purchase,
    Refund,
}

impl FromStr for TransactionType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "deposit" => Ok(TransactionType::Deposit),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            "purchase" => Ok(TransactionType::Purchase),
            "refund" => Ok(TransactionType::Refund),
            _ => Err(format!("Invalid transaction type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}

impl FromStr for TransactionStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(TransactionStatus::Pending),
            "completed" => Ok(TransactionStatus::Completed),
            "failed" => Ok(TransactionStatus::Failed),
            "reversed" => Ok(TransactionStatus::Reversed),
            _ => Err(format!("Invalid transaction status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum PurchaseFlowStatus {
    Initiated,
    PendingApproval,
    Approved,
    Rejected,
    Completed,
    Failed,
}

impl FromStr for PurchaseFlowStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "initiated" => Ok(PurchaseFlowStatus::Initiated),
            "pendingapproval" => Ok(PurchaseFlowStatus::PendingApproval),
            "approved" => Ok(PurchaseFlowStatus::Approved),
            "rejected" => Ok(PurchaseFlowStatus::Rejected),
            "completed" => Ok(PurchaseFlowStatus::Completed),
            "failed" => Ok(PurchaseFlowStatus::Failed),
            _ => Err(format!("Invalid purchase flow status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum RefundStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

impl FromStr for RefundStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(RefundStatus::Pending),
            "approved" => Ok(RefundStatus::Approved),
            "rejected" => Ok(RefundStatus::Rejected),
            "completed" => Ok(RefundStatus::Completed),
            _ => Err(format!("Invalid refund status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum AdminActionType {
    ApproveCharge,
    RejectCharge,
    ReverseTransaction,
    SuspendWallet,
    ActivateWallet,
}

impl FromStr for AdminActionType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "approvecharge" => Ok(AdminActionType::ApproveCharge),
            "rejectcharge" => Ok(AdminActionType::RejectCharge),
            "reversetransaction" => Ok(AdminActionType::ReverseTransaction),
            "suspendwallet" => Ok(AdminActionType::SuspendWallet),
            "activatewallet" => Ok(AdminActionType::ActivateWallet),
            _ => Err(format!("Invalid admin action type: {}", s)),
        }
    }
}

// --- Structs ---

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: i64,
    pub currency: String,
    pub status: WalletStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PurchaseFlow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: i64,
    pub status: PurchaseFlowStatus,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub wallet_id: Uuid,
    #[sqlx(rename = "type")]
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub status: TransactionStatus,
    pub description: Option<String>,
    pub reference_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefundRequest {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub amount: i64,
    pub reason: Option<String>,
    pub status: RefundStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AdminAction {
    pub id: Uuid,
    pub admin_id: Uuid,
    #[sqlx(rename = "action_type")]
    pub admin_action_type: AdminActionType,
    pub target_id: Uuid,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

// --- DTOs (Data Transfer Objects) for API requests/responses ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: i64,
    pub currency: String,
    pub status: WalletStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub status: TransactionStatus,
    pub description: Option<String>,
    pub reference_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseFlowResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: i64,
    pub status: PurchaseFlowStatus,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequestResponse {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub amount: i64,
    pub reason: Option<String>,
    pub status: RefundStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionResponse {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub admin_action_type: AdminActionType,
    pub target_id: Uuid,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWalletRequest {
    pub status: Option<WalletStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub wallet_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub description: Option<String>,
    pub reference_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTransactionStatusRequest {
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePurchaseFlowRequest {
    pub user_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: i64,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePurchaseFlowStatusRequest {
    pub status: PurchaseFlowStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefundRequest {
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub amount: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRefundRequestStatus {
    pub status: RefundStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAdminActionRequest {
    pub admin_id: Uuid,
    pub admin_action_type: AdminActionType,
    pub target_id: Uuid,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

