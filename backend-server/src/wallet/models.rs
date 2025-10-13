use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Re-export enums and structs from shared models to avoid duplication
pub use models::wallet::{WalletStatus, Wallet, CreateWallet};
pub use models::transaction::{TransactionType, TransactionStatus, Transaction};

// --- DTOs (Data Transfer Objects) for API requests/responses ---

// TODO: Add these DTOs to the models crate
// pub use models::wallet::{CreateWalletRequest, WalletResponse, TransactionResponse, PurchaseFlowResponse, RefundRequestResponse, AdminActionResponse, UpdateWalletRequest, CreateTransactionRequest, UpdateTransactionStatusRequest, CreatePurchaseFlowRequest, UpdatePurchaseFlowStatusRequest, CreateRefundRequest, UpdateRefundRequestStatus, CreateAdminActionRequest};

