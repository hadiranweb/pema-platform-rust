use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::wallet::WalletStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWallet {
    pub user_id: Uuid,
    pub balance: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWallet {
    pub balance: Option<f64>,
    pub status: Option<WalletStatus>,
}

