
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt::{self, Display};
use std::str::FromStr;

#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), sqlx(type_name = "transaction_type", rename_all = "PascalCase"))]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    TransferIn,
    TransferOut,
    Purchase,
    Refund,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "Deposit"),
            TransactionType::Withdrawal => write!(f, "Withdrawal"),
            TransactionType::TransferIn => write!(f, "TransferIn"),
            TransactionType::TransferOut => write!(f, "TransferOut"),
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
            "TransferIn" => Ok(TransactionType::TransferIn),
            "TransferOut" => Ok(TransactionType::TransferOut),
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTransaction {
    pub wallet_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub description: Option<String>,
    pub reference_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateTransaction {
    pub transaction_type: Option<TransactionType>,
    pub amount: Option<f64>,
    pub status: Option<TransactionStatus>,
    pub description: Option<String>,
    pub reference_id: Option<Uuid>,
}

