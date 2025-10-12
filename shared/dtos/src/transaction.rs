use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

