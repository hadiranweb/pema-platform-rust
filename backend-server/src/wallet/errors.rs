use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum WalletError {
    #[error("Wallet not found for user: {0}")]
    WalletNotFound(uuid::Uuid),
    #[error("Insufficient funds. Required: {required}, Available: {available}")]
    InsufficientFunds {
        required: f64,
        available: f64,
    },
    #[error("Transaction not found: {0}")]
    TransactionNotFound(uuid::Uuid),
    #[error("Purchase flow not found: {0}")]
    PurchaseFlowNotFound(uuid::Uuid),
    #[error("Refund request not found: {0}")]
    RefundRequestNotFound(uuid::Uuid),
    #[error("Invalid transaction state for operation")]
    InvalidTransactionState,
    #[error("Invalid purchase flow status transition")]
    InvalidPurchaseFlowStatusTransition,
    #[error("Invalid refund request status transition")]
    InvalidRefundRequestStatusTransition,
    #[error("Transaction already processed")]
    TransactionAlreadyProcessed,
    #[error("Admin action unauthorized: {0}")]
    UnauthorizedAdminAction(String),
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Duplicate reference number: {0}")]
    DuplicateReferenceNumber(String),
    #[error("Wallet balance limit exceeded")]
    BalanceLimitExceeded,
    #[error("Payment gateway error: {0}")]
    PaymentGatewayError(String),
    #[error("Invalid WalletStatus: {0}")]
    InvalidWalletStatus(String),
    #[error("Invalid TransactionType: {0}")]
    InvalidTransactionType(String),
    #[error("Invalid TransactionStatus: {0}")]
    InvalidTransactionStatus(String),
    #[error("Invalid PurchaseFlowStatus: {0}")]
    InvalidPurchaseFlowStatus(String),
    #[error("Invalid RefundStatus: {0}")]
    InvalidRefundStatus(String),
    #[error("Invalid AdminActionType: {0}")]
    InvalidAdminActionType(String),
}

impl ResponseError for WalletError {
    fn error_response(&self) -> HttpResponse {
        #[derive(Serialize)]
        struct ErrorResponse {
            code: String,
            message: String,
        }

        let (status, code) = match self {
            WalletError::WalletNotFound(_) => (StatusCode::NOT_FOUND, "WALLET_NOT_FOUND"),
            WalletError::InsufficientFunds { .. } => (StatusCode::BAD_REQUEST, "INSUFFICIENT_FUNDS"),
            WalletError::TransactionNotFound(_) => (StatusCode::NOT_FOUND, "TRANSACTION_NOT_FOUND"),
            WalletError::PurchaseFlowNotFound(_) => (StatusCode::NOT_FOUND, "PURCHASE_FLOW_NOT_FOUND"),
            WalletError::RefundRequestNotFound(_) => (StatusCode::NOT_FOUND, "REFUND_REQUEST_NOT_FOUND"),
            WalletError::InvalidTransactionState => (StatusCode::BAD_REQUEST, "INVALID_TRANSACTION_STATE"),
            WalletError::InvalidPurchaseFlowStatusTransition => (StatusCode::BAD_REQUEST, "INVALID_PURCHASE_FLOW_STATUS_TRANSITION"),
            WalletError::InvalidRefundRequestStatusTransition => (StatusCode::BAD_REQUEST, "INVALID_REFUND_REQUEST_STATUS_TRANSITION"),
            WalletError::TransactionAlreadyProcessed => (StatusCode::CONFLICT, "TRANSACTION_ALREADY_PROCESSED"),
            WalletError::UnauthorizedAdminAction(_) => (StatusCode::FORBIDDEN, "UNAUTHORIZED_ADMIN_ACTION"),
            WalletError::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DB_ERROR"),
            WalletError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
            WalletError::InvalidInput(_) => (StatusCode::BAD_REQUEST, "INVALID_INPUT"),
            WalletError::DuplicateReferenceNumber(_) => (StatusCode::CONFLICT, "DUPLICATE_REFERENCE_NUMBER"),
            WalletError::BalanceLimitExceeded => (StatusCode::BAD_REQUEST, "BALANCE_LIMIT_EXCEEDED"),
            WalletError::PaymentGatewayError(_) => (StatusCode::BAD_GATEWAY, "PAYMENT_GATEWAY_ERROR"),
            WalletError::InvalidWalletStatus(_) => (StatusCode::BAD_REQUEST, "INVALID_WALLET_STATUS"),
            WalletError::InvalidTransactionType(_) => (StatusCode::BAD_REQUEST, "INVALID_TRANSACTION_TYPE"),
            WalletError::InvalidTransactionStatus(_) => (StatusCode::BAD_REQUEST, "INVALID_TRANSACTION_STATUS"),
            WalletError::InvalidPurchaseFlowStatus(_) => (StatusCode::BAD_REQUEST, "INVALID_PURCHASE_FLOW_STATUS"),
            WalletError::InvalidRefundStatus(_) => (StatusCode::BAD_REQUEST, "INVALID_REFUND_STATUS"),
            WalletError::InvalidAdminActionType(_) => (StatusCode::BAD_REQUEST, "INVALID_ADMIN_ACTION_TYPE"),
        };

        HttpResponse::build(status).json(ErrorResponse {
            code: code.to_string(),
            message: self.to_string(),
        })
    }
}

impl From<sqlx::Error> for WalletError {
    fn from(err: sqlx::Error) -> Self {
        log::error!("SQLx Error: {:?}", err);
        match err {
            sqlx::Error::Database(db_err) => {
                if let Some(code) = db_err.code() {
                    if code.as_ref() == "23505" { // Unique violation
                        if db_err.message().contains("reference_number") {
                            return WalletError::DuplicateReferenceNumber(db_err.message().to_string());
                        }
                    }
                }
                WalletError::DbError(db_err.to_string())
            }
            _ => WalletError::DbError(err.to_string()),
        }
    }
}

impl From<std::io::Error> for WalletError {
    fn from(err: std::io::Error) -> Self {
        WalletError::InternalError(format!("IO Error: {}", err))
    }
}

impl From<serde_json::Error> for WalletError {
    fn from(err: serde_json::Error) -> Self {
        WalletError::InternalError(format!("JSON Error: {}", err))
    }
}

impl From<std::string::ParseError> for WalletError {
    fn from(err: std::string::ParseError) -> Self {
        WalletError::InvalidInput(format!("Parse Error: {}", err))
    }
}

impl From<uuid::Error> for WalletError {
    fn from(err: uuid::Error) -> Self {
        WalletError::InvalidInput(format!("UUID Parse Error: {}", err))
    }
}

