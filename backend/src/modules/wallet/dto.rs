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

// If the backend needs to convert from its own validated DTOs to the shared DTOs, 
// these `From` implementations would be useful. Otherwise, the backend can directly use `dtos::wallet::CreateWallet`
// if the validation is moved to the shared DTOs.
// For now, keeping the validation in the backend's DTOs and providing conversion.
impl From<CreateWallet> for DtosCreateWallet {
    fn from(dto: CreateWallet) -> Self {
        Self {
            user_id: dto.user_id,
            balance: dto.balance,
            currency: dto.currency,
        }
    }
}

impl From<UpdateWallet> for DtosUpdateWallet {
    fn from(dto: UpdateWallet) -> Self {
        Self {
            balance: dto.balance,
            status: dto.status,
        }
    }
}

