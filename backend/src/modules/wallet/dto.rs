
pub use crate::shared::models::wallet::{CreateWallet, UpdateWallet, Wallet};
pub use crate::shared::models::transaction::{CreateTransaction, UpdateTransaction, Transaction};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositRequest {
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawRequest {
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequest {
    pub recipient_wallet_id: uuid::Uuid,
    pub amount: f64,
}

