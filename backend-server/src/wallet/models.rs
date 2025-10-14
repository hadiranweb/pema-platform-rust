
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

// --- Enums ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "wallet_status", rename_all = "lowercase")]
pub enum WalletStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Purchase,
    Refund,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "purchase_flow_status", rename_all = "lowercase")]
pub enum PurchaseFlowStatus {
    Initiated,
    PendingApproval,
    Approved,
    Rejected,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "refund_status", rename_all = "lowercase")]
pub enum RefundStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow, sqlx::Type)]
#[sqlx(type_name = "admin_action_type", rename_all = "lowercase")]
pub enum AdminActionType {
    ApproveCharge,
    RejectCharge,
    ReverseTransaction,
    SuspendWallet,
    ActivateWallet,
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
}

