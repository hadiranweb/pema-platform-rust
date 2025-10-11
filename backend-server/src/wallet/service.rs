use sqlx::PgPool;
use uuid::Uuid;

use log::info;

use crate::wallet::models::{Wallet, WalletStatus, Transaction, TransactionType, TransactionStatus, PurchaseFlow, PurchaseFlowStatus, RefundRequest, RefundStatus, AdminAction, AdminActionType, CreateWalletRequest, CreateTransactionRequest, CreatePurchaseFlowRequest, CreateRefundRequest, CreateAdminActionRequest};
use crate::wallet::errors::WalletError;

pub struct WalletService {
    pool: PgPool,
}

impl WalletService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // --- Wallet Management ---

    pub async fn create_wallet(&self, user_id: Uuid, new_wallet: CreateWalletRequest) -> Result<Wallet, WalletError> {
        let wallet = sqlx::query_as::<_, Wallet>(
            "INSERT INTO wallets (id, user_id, balance, currency, status, created_at, updated_at) VALUES (uuid_generate_v4(), $1, $2, $3, $4, now(), now()) RETURNING *"
        )
        .bind(user_id)
        .bind(new_wallet.initial_balance.unwrap_or(0.0))
        .bind(new_wallet.currency)
        .bind(WalletStatus::Active.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?;
        info!("New wallet created for user_id: {}", user_id);
        Ok(wallet)
    }

    pub async fn get_wallet_by_user_id(&self, user_id: Uuid) -> Result<Wallet, WalletError> {
        sqlx::query_as::<_, Wallet>(
            "SELECT * FROM wallets WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?
        .ok_or_else(|| WalletError::WalletNotFound(user_id))
    }

    pub async fn get_wallet_by_id(&self, wallet_id: Uuid) -> Result<Wallet, WalletError> {
        sqlx::query_as::<_, Wallet>(
            "SELECT * FROM wallets WHERE id = $1"
        )
        .bind(wallet_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?
        .ok_or_else(|| WalletError::WalletNotFound(wallet_id))
    }

    pub async fn update_wallet_status(&self, wallet_id: Uuid, status: String) -> Result<Wallet, WalletError> {
        let wallet = sqlx::query_as::<_, Wallet>(
            "UPDATE wallets SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(status)
        .bind(wallet_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?;
        info!("Wallet {} status updated to {}", wallet_id, wallet.status);
        Ok(wallet)
    }

    // --- Transaction Processing ---

    pub async fn create_transaction(
        &self,
        new_transaction: CreateTransactionRequest,
    ) -> Result<Transaction, WalletError> {
        let mut tx = self.pool.begin().await.map_err(|e| WalletError::DbError(e.to_string()))?;

        let wallet = sqlx::query_as::<_, Wallet>(
            "SELECT * FROM wallets WHERE id = $1 FOR UPDATE"
        )
        .bind(new_transaction.wallet_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?
        .ok_or_else(|| WalletError::WalletNotFound(new_transaction.wallet_id))?;

        // Handle balance update based on transaction type
        let new_balance = match new_transaction.transaction_type.to_string().as_str() {
            "Deposit" => wallet.balance + new_transaction.amount,
            "Withdrawal" | "Purchase" | "Refund" => {
                if wallet.balance < new_transaction.amount {
                    tx.rollback().await.map_err(|e| WalletError::DbError(e.to_string()))?;
                    return Err(WalletError::InsufficientFunds { required: new_transaction.amount, available: wallet.balance });
                }
                wallet.balance - new_transaction.amount
            }
            _ => return Err(WalletError::InvalidTransactionType(new_transaction.transaction_type.to_string()))
        };

        sqlx::query!(
            "UPDATE wallets SET balance = $1, updated_at = NOW() WHERE id = $2",
            new_balance,
            new_transaction.wallet_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?;


        let transaction = sqlx::query_as::<_, Transaction>(
            "INSERT INTO transactions (id, wallet_id, transaction_type, amount, status, description, reference_id, created_at, updated_at) VALUES (uuid_generate_v4(), $1, $2, $3, $4, $5, $6, now(), now()) RETURNING *"
        )
        .bind(new_transaction.wallet_id)
        .bind(new_transaction.transaction_type.to_string())
        .bind(new_transaction.amount)
        .bind(TransactionStatus::Completed.to_string()) // Assuming immediate completion for now
        .bind(new_transaction.description)
        .bind(new_transaction.reference_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?;

        tx.commit().await.map_err(|e| WalletError::DbError(e.to_string()))?;
        info!("Transaction {:?} of {} for wallet {} completed.", transaction.transaction_type, transaction.amount, new_transaction.wallet_id);
        Ok(transaction)
    }

    pub async fn get_transactions_by_wallet_id(&self, wallet_id: Uuid) -> Result<Vec<Transaction>, WalletError> {
        sqlx::query_as::<_, Transaction>(
            "SELECT * FROM transactions WHERE wallet_id = $1 ORDER BY created_at DESC"
        )
        .bind(wallet_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))
    }

    pub async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Transaction, WalletError> {
        sqlx::query_as::<_, Transaction>(
            "SELECT * FROM transactions WHERE id = $1"
        )
        .bind(transaction_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?
        .ok_or_else(|| WalletError::TransactionNotFound(transaction_id))
    }

    pub async fn update_transaction_status(&self, transaction_id: Uuid, status: String) -> Result<Transaction, WalletError> {
        let transaction = sqlx::query_as::<_, Transaction>(
            "UPDATE transactions SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(status)
        .bind(transaction_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?;
        info!("Transaction {} status updated to {}", transaction_id, transaction.status);
        Ok(transaction)
    }

    // --- Purchase Flow Management ---

    pub async fn create_purchase_flow(
        &self,
        new_purchase_flow: CreatePurchaseFlowRequest,
    ) -> Result<PurchaseFlow, WalletError> {
        let purchase_flow = sqlx::query_as::<_, PurchaseFlow>(
            "INSERT INTO purchase_flows (id, user_id, wallet_id, amount, status, created_at, updated_at) VALUES (uuid_generate_v4(), $1, $2, $3, $4, now(), now()) RETURNING *"
        )
        .bind(new_purchase_flow.user_id)
        .bind(new_purchase_flow.wallet_id)
        .bind(new_purchase_flow.amount)
        .bind(PurchaseFlowStatus::Initiated.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?;
        info!("New purchase flow {} initiated for user {}.", purchase_flow.id, new_purchase_flow.user_id);
        Ok(purchase_flow)
    }

    pub async fn get_purchase_flow_by_id(&self, flow_id: Uuid) -> Result<PurchaseFlow, WalletError> {
        sqlx::query_as::<_, PurchaseFlow>(
            "SELECT * FROM purchase_flows WHERE id = $1"
        )
        .bind(flow_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?
        .ok_or_else(|| WalletError::PurchaseFlowNotFound(flow_id))
    }

    pub async fn update_purchase_flow_status(&self, flow_id: Uuid, status: String) -> Result<PurchaseFlow, WalletError> {
        let purchase_flow = sqlx::query_as::<_, PurchaseFlow>(
            "UPDATE purchase_flows SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(status)
        .bind(flow_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?;
        info!("Purchase flow {} status updated to {}", flow_id, purchase_flow.status);
        Ok(purchase_flow)
    }

    // --- Refund Request Management ---

    pub async fn create_refund_request(
        &self,
        new_refund_request: CreateRefundRequest,
    ) -> Result<RefundRequest, WalletError> {
        let refund_request = sqlx::query_as::<_, RefundRequest>(
            "INSERT INTO refund_requests (id, transaction_id, user_id, amount, reason, status, created_at, updated_at) VALUES (uuid_generate_v4(), $1, $2, $3, $4, $5, now(), now()) RETURNING *"
        )
        .bind(new_refund_request.transaction_id)
        .bind(new_refund_request.user_id)
        .bind(new_refund_request.amount)
        .bind(new_refund_request.reason)
        .bind(RefundStatus::Pending.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?;
        info!("New refund request {} created for transaction {}.", refund_request.id, new_refund_request.transaction_id);
        Ok(refund_request)
    }

    pub async fn get_refund_request_by_id(&self, request_id: Uuid) -> Result<RefundRequest, WalletError> {
        sqlx::query_as::<_, RefundRequest>(
            "SELECT * FROM refund_requests WHERE id = $1"
        )
        .bind(request_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?
        .ok_or_else(|| WalletError::RefundRequestNotFound(request_id))
    }

    pub async fn update_refund_request_status(&self, request_id: Uuid, status: String) -> Result<RefundRequest, WalletError> {
        let refund_request = sqlx::query_as::<_, RefundRequest>(
            "UPDATE refund_requests SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(status)
        .bind(request_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?;
        info!("Refund request {} status updated to {}", request_id, refund_request.status);
        Ok(refund_request)
    }

    // --- Admin Actions ---

    pub async fn record_admin_action(
        &self,
        new_admin_action: CreateAdminActionRequest,
    ) -> Result<AdminAction, WalletError> {
        let admin_action = sqlx::query_as::<_, AdminAction>(
            "INSERT INTO admin_actions (id, admin_id, action_type, target_id, created_at) VALUES (uuid_generate_v4(), $1, $2, $3, now()) RETURNING *"
        )
        .bind(new_admin_action.admin_id)
        .bind(new_admin_action.action_type.to_string())
        .bind(new_admin_action.target_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))?;
        info!("Admin {} performed action {:?} on target {}.", new_admin_action.admin_id, admin_action.action_type, new_admin_action.target_id);
        Ok(admin_action)
    }

    pub async fn get_admin_actions_by_admin_id(&self, admin_id: Uuid) -> Result<Vec<AdminAction>, WalletError> {
        sqlx::query_as::<_, AdminAction>(
            "SELECT * FROM admin_actions WHERE admin_id = $1 ORDER BY created_at DESC"
        )
        .bind(admin_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))
    }

    pub async fn get_admin_actions_by_target_id(&self, target_id: Uuid) -> Result<Vec<AdminAction>, WalletError> {
        sqlx::query_as::<_, AdminAction>(
            "SELECT * FROM admin_actions WHERE target_id = $1 ORDER BY created_at DESC"
        )
        .bind(target_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| WalletError::DbError(e.to_string()))
    }
}

