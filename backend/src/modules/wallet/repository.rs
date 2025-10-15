
use sqlx::{PgPool, Postgres, Transaction as DbTransaction};
use uuid::Uuid;

use models::wallet::{CreateWallet, UpdateWallet, Wallet};
use models::transaction::{CreateTransaction, Transaction};

// Wallet Operations
pub async fn create_wallet(pool: &PgPool, create_wallet: CreateWallet) -> Result<Wallet, String> {
    sqlx::query_as::<_, Wallet>(
        "INSERT INTO wallets (id, user_id, balance, currency) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(create_wallet.user_id)
    .bind(create_wallet.balance)
    .bind(create_wallet.currency)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to create wallet: {}", e))
}

pub async fn find_wallet_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Wallet, String> {
    sqlx::query_as::<_, Wallet>("SELECT * FROM wallets WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch wallet by user ID: {}", e))?
        .ok_or_else(|| "Wallet not found for this user".to_string())
}

pub async fn find_wallet_by_id(executor: &mut DbTransaction<'_, Postgres>, wallet_id: Uuid) -> Result<Wallet, String> {
    sqlx::query_as::<_, Wallet>("SELECT * FROM wallets WHERE id = $1 FOR UPDATE")
        .bind(wallet_id)
        .fetch_optional(executor)
        .await
        .map_err(|e| format!("Failed to fetch wallet by ID: {}", e))?
        .ok_or_else(|| "Wallet not found".to_string())
}

pub async fn find_wallet_by_id_from_pool(pool: &PgPool, wallet_id: Uuid) -> Result<Wallet, String> {
    sqlx::query_as::<_, Wallet>("SELECT * FROM wallets WHERE id = $1")
        .bind(wallet_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch wallet by ID from pool: {}", e))?
        .ok_or_else(|| "Wallet not found".to_string())
}

pub async fn update_wallet_balance(executor: &mut DbTransaction<'_, Postgres>, user_id: Uuid, amount: f64) -> Result<Wallet, String> {
    let wallet = sqlx::query_as::<_, Wallet>(
        "UPDATE wallets SET balance = balance + $1, updated_at = NOW() WHERE user_id = $2 RETURNING *"
    )
    .bind(amount)
    .bind(user_id)
    .fetch_optional(executor)
    .await
    .map_err(|e| format!("Failed to update wallet balance: {}", e))?
    .ok_or_else(|| "Wallet not found or insufficient funds".to_string());

    // Check for insufficient funds after the update attempt
    if let Ok(ref w) = wallet {
        if w.balance < 0.0 {
            return Err("Insufficient funds".to_string());
        }
    }
    wallet
}

pub async fn update_wallet_balance_by_id(executor: &mut DbTransaction<'_, Postgres>, wallet_id: Uuid, amount: f64) -> Result<Wallet, String> {
    let wallet = sqlx::query_as::<_, Wallet>(
        "UPDATE wallets SET balance = balance + $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(amount)
    .bind(wallet_id)
    .fetch_optional(executor)
    .await
    .map_err(|e| format!("Failed to update wallet balance by ID: {}", e))?
    .ok_or_else(|| "Wallet not found or insufficient funds".to_string());

    if let Ok(ref w) = wallet {
        if w.balance < 0.0 {
            return Err("Insufficient funds".to_string());
        }
    }
    wallet
}

// Transaction Operations
pub async fn create_transaction(executor: &mut DbTransaction<'_, Postgres>, create_transaction: CreateTransaction) -> Result<Transaction, String> {
    sqlx::query_as::<_, Transaction>(
        "INSERT INTO transactions (id, wallet_id, transaction_type, amount, status, description, reference_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(create_transaction.wallet_id)
    .bind(create_transaction.transaction_type)
    .bind(create_transaction.amount)
    .bind(models::transaction::TransactionStatus::Completed) // Default to completed for now
    .bind(create_transaction.description)
    .bind(create_transaction.reference_id)
    .fetch_one(executor)
    .await
    .map_err(|e| format!("Failed to create transaction: {}", e))
}

pub async fn find_transactions_by_wallet_id(pool: &PgPool, wallet_id: Uuid) -> Result<Vec<Transaction>, String> {
    sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE wallet_id = $1 ORDER BY created_at DESC")
        .bind(wallet_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch transactions for wallet: {}", e))
}

