use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Re-export enums and structs from shared models to avoid duplication
pub use models::wallet::{WalletStatus, TransactionType, TransactionStatus, PurchaseFlowStatus, RefundStatus, AdminActionType, Wallet, PurchaseFlow, Transaction, RefundRequest, AdminAction};

// --- DTOs (Data Transfer Objects) for API requests/responses ---

pub use models::wallet::{CreateWalletRequest, WalletResponse, TransactionResponse, PurchaseFlowResponse, RefundRequestResponse, AdminActionResponse, UpdateWalletRequest, CreateTransactionRequest, UpdateTransactionStatusRequest, CreatePurchaseFlowRequest, UpdatePurchaseFlowStatusRequest, CreateRefundRequest, UpdateRefundRequestStatus, CreateAdminActionRequest};

