use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use std::fmt::{self, Display};
use std::str::FromStr;

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), sqlx(type_name = "wallet_status", rename_all = "PascalCase"))]
pub enum WalletStatus {
    Active,
    Inactive,
    Suspended,
    Closed,
}

impl Display for WalletStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WalletStatus::Active => write!(f, "Active"),
            WalletStatus::Inactive => write!(f, "Inactive"),
            WalletStatus::Suspended => write!(f, "Suspended"),
            WalletStatus::Closed => write!(f, "Closed"),
        }
    }
}

impl FromStr for WalletStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(WalletStatus::Active),
            "Inactive" => Ok(WalletStatus::Inactive),
            "Suspended" => Ok(WalletStatus::Suspended),
            "Closed" => Ok(WalletStatus::Closed),
            _ => Err(format!("Invalid WalletStatus: {}", s)),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), sqlx(type_name = "transaction_type", rename_all = "PascalCase"))]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
    Purchase,
    Refund,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "Deposit"),
            TransactionType::Withdrawal => write!(f, "Withdrawal"),
            TransactionType::Transfer => write!(f, "Transfer"),
            TransactionType::Purchase => write!(f, "Purchase"),
            TransactionType::Refund => write!(f, "Refund"),
        }
    }
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Deposit" => Ok(TransactionType::Deposit),
            "Withdrawal" => Ok(TransactionType::Withdrawal),
            "Transfer" => Ok(TransactionType::Transfer),
            "Purchase" => Ok(TransactionType::Purchase),
            "Refund" => Ok(TransactionType::Refund),
            _ => Err(format!("Invalid TransactionType: {}", s)),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), sqlx(type_name = "transaction_status", rename_all = "PascalCase"))]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "Pending"),
            TransactionStatus::Completed => write!(f, "Completed"),
            TransactionStatus::Failed => write!(f, "Failed"),
            TransactionStatus::Reversed => write!(f, "Reversed"),
        }
    }
}

impl FromStr for TransactionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(TransactionStatus::Pending),
            "Completed" => Ok(TransactionStatus::Completed),
            "Failed" => Ok(TransactionStatus::Failed),
            "Reversed" => Ok(TransactionStatus::Reversed),
            _ => Err(format!("Invalid TransactionStatus: {}", s)),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), sqlx(type_name = "purchase_flow_status", rename_all = "PascalCase"))]
pub enum PurchaseFlowStatus {
    Initiated,
    PendingApproval,
    Approved,
    Rejected,
    Completed,
    Failed,
}

impl Display for PurchaseFlowStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PurchaseFlowStatus::Initiated => write!(f, "Initiated"),
            PurchaseFlowStatus::PendingApproval => write!(f, "PendingApproval"),
            PurchaseFlowStatus::Approved => write!(f, "Approved"),
            PurchaseFlowStatus::Rejected => write!(f, "Rejected"),
            PurchaseFlowStatus::Completed => write!(f, "Completed"),
            PurchaseFlowStatus::Failed => write!(f, "Failed"),
        }
    }
}

impl FromStr for PurchaseFlowStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Initiated" => Ok(PurchaseFlowStatus::Initiated),
            "PendingApproval" => Ok(PurchaseFlowStatus::PendingApproval),
            "Approved" => Ok(PurchaseFlowStatus::Approved),
            "Rejected" => Ok(PurchaseFlowStatus::Rejected),
            "Completed" => Ok(PurchaseFlowStatus::Completed),
            "Failed" => Ok(PurchaseFlowStatus::Failed),
            _ => Err(format!("Invalid PurchaseFlowStatus: {}", s)),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), sqlx(type_name = "refund_status", rename_all = "PascalCase"))]
pub enum RefundStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

impl Display for RefundStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RefundStatus::Pending => write!(f, "Pending"),
            RefundStatus::Approved => write!(f, "Approved"),
            RefundStatus::Rejected => write!(f, "Rejected"),
            RefundStatus::Completed => write!(f, "Completed"),
        }
    }
}

impl FromStr for RefundStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(RefundStatus::Pending),
            "Approved" => Ok(RefundStatus::Approved),
            "Rejected" => Ok(RefundStatus::Rejected),
            "Completed" => Ok(RefundStatus::Completed),
            _ => Err(format!("Invalid RefundStatus: {}", s)),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), sqlx(type_name = "admin_action_type", rename_all = "PascalCase"))]
pub enum AdminActionType {
    ApproveCharge,
    RejectCharge,
    ReverseTransaction,
    SuspendWallet,
    ActivateWallet,
}

impl Display for AdminActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdminActionType::ApproveCharge => write!(f, "ApproveCharge"),
            AdminActionType::RejectCharge => write!(f, "RejectCharge"),
            AdminActionType::ReverseTransaction => write!(f, "ReverseTransaction"),
            AdminActionType::SuspendWallet => write!(f, "SuspendWallet"),
            AdminActionType::ActivateWallet => write!(f, "ActivateWallet"),
        }
    }
}

impl FromStr for AdminActionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ApproveCharge" => Ok(AdminActionType::ApproveCharge),
            "RejectCharge" => Ok(AdminActionType::RejectCharge),
            "ReverseTransaction" => Ok(AdminActionType::ReverseTransaction),
            "SuspendWallet" => Ok(AdminActionType::SuspendWallet),
            "ActivateWallet" => Ok(AdminActionType::ActivateWallet),
            _ => Err(format!("Invalid AdminActionType: {}", s)),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: f64,
    pub currency: String,
    pub status: WalletStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseFlow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: f64,
    pub status: PurchaseFlowStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub status: TransactionStatus,
    pub description: Option<String>,
    pub reference_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundRequest {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub reason: Option<String>,
    pub status: RefundStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdminAction {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub action_type: AdminActionType,
    pub target_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// --- DTOs (Data Transfer Objects) for API requests/responses ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub currency: String,
    pub initial_balance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: f64,
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
    pub amount: f64,
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
    pub amount: f64,
    pub status: PurchaseFlowStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequestResponse {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub reason: Option<String>,
    pub status: RefundStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionResponse {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub action_type: AdminActionType,
    pub target_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWalletRequest {
    pub status: WalletStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub wallet_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: f64,
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
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePurchaseFlowStatusRequest {
    pub status: PurchaseFlowStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefundRequest {
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRefundRequestStatus {
    pub status: RefundStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAdminActionRequest {
    pub admin_id: Uuid,
    pub action_type: AdminActionType,
    pub target_id: Uuid,
}

