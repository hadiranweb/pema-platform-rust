use sqlx::{PgPool, Transaction as DbTransaction};
use uuid::Uuid;

use models::wallet::{CreateWallet, UpdateWallet, Wallet};
use models::transaction::{CreateTransaction, Transaction, TransactionType};
use crate::modules::wallet::repository;
use crate::modules::wallet::dto::{DepositRequest, WithdrawRequest, TransferRequest};
use crate::error::ServiceError;

pub struct WalletService;

impl WalletService {
    pub async fn create_wallet(pool: &PgPool, create_wallet: CreateWallet) -> Result<Wallet, ServiceError> {
        repository::create_wallet(pool, create_wallet).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn get_wallet_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Wallet, ServiceError> {
        repository::find_wallet_by_user_id(pool, user_id).await.map_err(ServiceError::NotFound)
    }

    pub async fn get_wallet_by_id(pool: &PgPool, wallet_id: Uuid) -> Result<Wallet, ServiceError> {
        repository::find_wallet_by_id_from_pool(pool, wallet_id).await.map_err(ServiceError::NotFound)
    }
    pub async fn get_transactions_by_wallet_id(pool: &PgPool, wallet_id: Uuid) -> Result<Vec<Transaction>, ServiceError> {
        repository::find_transactions_by_wallet_id(pool, wallet_id).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn deposit(pool: &PgPool, user_id: Uuid, request: DepositRequest) -> Result<Wallet, ServiceError> {
        if request.amount <= 0.0 {
            return Err(ServiceError::BadRequest("Deposit amount must be positive".to_string()));
        }

        let mut tx = pool.begin().await.map_err(|e| ServiceError::DatabaseError(format!("Failed to begin transaction: {}", e)))?;

        let wallet = repository::update_wallet_balance(&mut tx, user_id, request.amount).await.map_err(ServiceError::DatabaseError)?;

        let create_transaction = CreateTransaction {
            wallet_id: wallet.id,
            transaction_type: TransactionType::Deposit,
            amount: request.amount,
            description: Some("Deposit to wallet".to_string()),
        };
        repository::create_transaction(&mut tx, create_transaction).await.map_err(ServiceError::DatabaseError)?;

        tx.commit().await.map_err(|e| ServiceError::DatabaseError(format!("Failed to commit transaction: {}", e)))?;
        Ok(wallet)
    }

    pub async fn withdraw(pool: &PgPool, user_id: Uuid, request: WithdrawRequest) -> Result<Wallet, ServiceError> {
        if request.amount <= 0.0 {
            return Err(ServiceError::BadRequest("Withdrawal amount must be positive".to_string()));
        }

        let mut tx = pool.begin().await.map_err(|e| ServiceError::DatabaseError(format!("Failed to begin transaction: {}", e)))?;

        let wallet = repository::update_wallet_balance(&mut tx, user_id, -request.amount).await.map_err(ServiceError::DatabaseError)?;

        let create_transaction = CreateTransaction {
            wallet_id: wallet.id,
            transaction_type: TransactionType::Withdrawal,
            amount: request.amount,
            description: Some("Withdrawal from wallet".to_string()),
        };
        repository::create_transaction(&mut tx, create_transaction).await.map_err(ServiceError::DatabaseError)?;

        tx.commit().await.map_err(|e| ServiceError::DatabaseError(format!("Failed to commit transaction: {}", e)))?;
        Ok(wallet)
    }

    pub async fn transfer(pool: &PgPool, sender_user_id: Uuid, request: TransferRequest) -> Result<Wallet, ServiceError> {
        if request.amount <= 0.0 {
            return Err(ServiceError::BadRequest("Transfer amount must be positive".to_string()));
        }

        let mut tx = pool.begin().await.map_err(|e| ServiceError::DatabaseError(format!("Failed to begin transaction: {}", e)))?;

        // Deduct from sender
        let sender_wallet = repository::update_wallet_balance(&mut tx, sender_user_id, -request.amount).await.map_err(ServiceError::DatabaseError)?;

        // Add to recipient
        let recipient_wallet = repository::find_wallet_by_id(&mut tx, request.recipient_wallet_id).await.map_err(ServiceError::NotFound)?;
        repository::update_wallet_balance_by_id(&mut tx, recipient_wallet.id, request.amount).await.map_err(ServiceError::DatabaseError)?;

        // Create transactions for both sender and recipient
        let sender_transaction = CreateTransaction {
            wallet_id: sender_wallet.id,
            transaction_type: TransactionType::TransferOut,
            amount: request.amount,
            description: Some(format!("Transfer to wallet {}", recipient_wallet.id)),
        };
        repository::create_transaction(&mut tx, sender_transaction).await.map_err(ServiceError::DatabaseError)?;

        let recipient_transaction = CreateTransaction {
            wallet_id: recipient_wallet.id,
            transaction_type: TransactionType::TransferIn,
            amount: request.amount,
            description: Some(format!("Transfer from wallet {}", sender_wallet.id)),
        };
        repository::create_transaction(&mut tx, recipient_transaction).await.map_err(ServiceError::DatabaseError)?;

        tx.commit().await.map_err(|e| ServiceError::DatabaseError(format!("Failed to commit transaction: {}", e)))?;
        Ok(sender_wallet)
    }
}

