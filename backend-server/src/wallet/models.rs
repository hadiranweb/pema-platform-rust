
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

// --- Enums ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalletStatus {
    Active,
    Inactive,
    Suspended,
}

impl WalletStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            WalletStatus::Active => "active",
            WalletStatus::Inactive => "inactive", 
            WalletStatus::Suspended => "suspended",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "active" => Ok(WalletStatus::Active),
            "inactive" => Ok(WalletStatus::Inactive),
            "suspended" => Ok(WalletStatus::Suspended),
            _ => Err(format!("Invalid wallet status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Purchase,
    Refund,
}

impl TransactionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Deposit => "deposit",
            TransactionType::Withdrawal => "withdrawal",
            TransactionType::Purchase => "purchase",
            TransactionType::Refund => "refund",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "deposit" => Ok(TransactionType::Deposit),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            "purchase" => Ok(TransactionType::Purchase),
            "refund" => Ok(TransactionType::Refund),
            _ => Err(format!("Invalid transaction type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}

impl TransactionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionStatus::Pending => "pending",
            TransactionStatus::Completed => "completed",
            TransactionStatus::Failed => "failed",
            TransactionStatus::Reversed => "reversed",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pending" => Ok(TransactionStatus::Pending),
            "completed" => Ok(TransactionStatus::Completed),
            "failed" => Ok(TransactionStatus::Failed),
            "reversed" => Ok(TransactionStatus::Reversed),
            _ => Err(format!("Invalid transaction status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PurchaseFlowStatus {
    Initiated,
    PendingApproval,
    Approved,
    Rejected,
    Completed,
    Failed,
}

impl PurchaseFlowStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PurchaseFlowStatus::Initiated => "initiated",
            PurchaseFlowStatus::PendingApproval => "pending_approval",
            PurchaseFlowStatus::Approved => "approved",
            PurchaseFlowStatus::Rejected => "rejected",
            PurchaseFlowStatus::Completed => "completed",
            PurchaseFlowStatus::Failed => "failed",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "initiated" => Ok(PurchaseFlowStatus::Initiated),
            "pending_approval" => Ok(PurchaseFlowStatus::PendingApproval),
            "approved" => Ok(PurchaseFlowStatus::Approved),
            "rejected" => Ok(PurchaseFlowStatus::Rejected),
            "completed" => Ok(PurchaseFlowStatus::Completed),
            "failed" => Ok(PurchaseFlowStatus::Failed),
            _ => Err(format!("Invalid purchase flow status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefundStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

impl RefundStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            RefundStatus::Pending => "pending",
            RefundStatus::Approved => "approved",
            RefundStatus::Rejected => "rejected",
            RefundStatus::Completed => "completed",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pending" => Ok(RefundStatus::Pending),
            "approved" => Ok(RefundStatus::Approved),
            "rejected" => Ok(RefundStatus::Rejected),
            "completed" => Ok(RefundStatus::Completed),
            _ => Err(format!("Invalid refund status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdminActionType {
    ApproveCharge,
    RejectCharge,
    ReverseTransaction,
    SuspendWallet,
    ActivateWallet,
}

impl AdminActionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AdminActionType::ApproveCharge => "approve_charge",
            AdminActionType::RejectCharge => "reject_charge",
            AdminActionType::ReverseTransaction => "reverse_transaction",
            AdminActionType::SuspendWallet => "suspend_wallet",
            AdminActionType::ActivateWallet => "activate_wallet",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "approve_charge" => Ok(AdminActionType::ApproveCharge),
            "reject_charge" => Ok(AdminActionType::RejectCharge),
            "reverse_transaction" => Ok(AdminActionType::ReverseTransaction),
            "suspend_wallet" => Ok(AdminActionType::SuspendWallet),
            "activate_wallet" => Ok(AdminActionType::ActivateWallet),
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
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PurchaseFlow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: i64,
    pub status: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub wallet_id: Uuid,
    #[sqlx(rename = "type")]
    pub transaction_type: String,
    pub amount: i64,
    pub status: String,
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
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AdminAction {
    pub id: Uuid,
    pub admin_id: Uuid,
    #[sqlx(rename = "action_type")]
    pub admin_action_type: String,
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
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub transaction_type: String,
    pub amount: i64,
    pub status: String,
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
    pub status: String,
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
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionResponse {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub admin_action_type: String,
    pub target_id: Uuid,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWalletRequest {
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub wallet_id: Uuid,
    pub transaction_type: String,
    pub amount: i64,
    pub description: Option<String>,
    pub reference_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTransactionStatusRequest {
    pub status: String,
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
    pub status: String,
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
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAdminActionRequest {
    pub admin_id: Uuid,
    pub admin_action_type: String,
    pub target_id: Uuid,
    pub details: Option<serde_json::Value>,
}

