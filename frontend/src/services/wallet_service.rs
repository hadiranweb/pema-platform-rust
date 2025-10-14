
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use gloo_storage::{LocalStorage, Storage};

use crate::utils::constants::TOKEN_KEY;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WalletStatus {
    Active,
    Inactive,
    Suspended,
    Closed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    TransferIn,
    TransferOut,
    Purchase,
    Refund,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: f64,
    pub currency: String,
    pub status: WalletStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub status: TransactionStatus,
    pub description: Option<String>,
    pub reference_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepositRequest {
    pub amount: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WithdrawRequest {
    pub amount: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferRequest {
    pub recipient_wallet_id: Uuid,
    pub amount: f64,
}

pub struct WalletService;

impl WalletService {
    fn get_token() -> Result<String, String> {
        LocalStorage::get(TOKEN_KEY).map_err(|_| "Token not found".to_string())
    }

    pub async fn get_my_wallet() -> Result<Wallet, String> {
        let token = Self::get_token()?;
        let response = Request::get("/api/wallet/me")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse wallet: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch wallet: {}", error_text))
        }
    }

    pub async fn get_wallet_transactions() -> Result<Vec<Transaction>, String> {
        let token = Self::get_token()?;
        let response = Request::get("/api/wallet/me/transactions")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse transactions: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch transactions: {}", error_text))
        }
    }

    pub async fn deposit_funds(request_body: DepositRequest) -> Result<Wallet, String> {
        let token = Self::get_token()?;
        let response = Request::post("/api/wallet/deposit")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&request_body)
            .map_err(|e| format!("Failed to serialize deposit request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse wallet after deposit: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Deposit failed: {}", error_text))
        }
    }

    pub async fn withdraw_funds(request_body: WithdrawRequest) -> Result<Wallet, String> {
        let token = Self::get_token()?;
        let response = Request::post("/api/wallet/withdraw")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&request_body)
            .map_err(|e| format!("Failed to serialize withdraw request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse wallet after withdrawal: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Withdrawal failed: {}", error_text))
        }
    }

    pub async fn transfer_funds(request_body: TransferRequest) -> Result<Wallet, String> {
        let token = Self::get_token()?;
        let response = Request::post("/api/wallet/transfer")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&request_body)
            .map_err(|e| format!("Failed to serialize transfer request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse wallet after transfer: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Transfer failed: {}", error_text))
        }
    }
}

