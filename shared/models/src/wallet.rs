use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use std::fmt::{self, Display};
use std::str::FromStr;

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "wallet_status", rename_all = "PascalCase"))]
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

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWallet {
    pub user_id: Uuid,
    pub balance: f64,
    pub currency: String,
}

