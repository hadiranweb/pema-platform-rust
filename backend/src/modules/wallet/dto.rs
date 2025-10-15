use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use dtos::wallet::{CreateWallet as DtosCreateWallet, UpdateWallet as DtosUpdateWallet};
use models::wallet::WalletStatus;

// Re-export core wallet DTOs from the shared dtos crate
pub use dtos::wallet::{CreateWallet, UpdateWallet};

// Specific DTOs for wallet operations that might include validation or specific fields not in shared DTOs
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DepositRequest {
    #[validate(range(min = 0.01))]
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct WithdrawRequest {
    #[validate(range(min = 0.01))]
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TransferRequest {
    #[validate(custom = "crate::utils::validators::is_uuid")]
    pub recipient_wallet_id: Uuid,
    #[validate(range(min = 0.01))]
    pub amount: f64,
}

// Note: Direct conversion implementations removed due to orphan rule.
// Use manual conversion or consider moving validation to shared DTOs.

