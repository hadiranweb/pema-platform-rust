
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::auth::middleware::AuthenticatedUser;
use crate::modules::wallet::dto::{DepositRequest, WithdrawRequest, TransferRequest};
use crate::modules::wallet::service::WalletService;

pub async fn get_my_wallet(pool: web::Data<PgPool>, auth_user: AuthenticatedUser) -> impl Responder {
    match WalletService::get_wallet_by_user_id(pool.get_ref(), auth_user.user_id).await {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => e.error_response(),
    }
}

pub async fn get_wallet_transactions(pool: web::Data<PgPool>, auth_user: AuthenticatedUser) -> impl Responder {
    match WalletService::get_wallet_by_user_id(pool.get_ref(), auth_user.user_id).await {
        Ok(wallet) => {
            match WalletService::get_transactions_by_wallet_id(pool.get_ref(), wallet.id).await {
                Ok(transactions) => HttpResponse::Ok().json(transactions),
                Err(e) => e.error_response(),
            }
        },
        Err(e) => e.error_response(),
    }
}

pub async fn deposit_funds(pool: web::Data<PgPool>, auth_user: AuthenticatedUser, request: web::Json<DepositRequest>) -> impl Responder {
    match WalletService::deposit(pool.get_ref(), auth_user.user_id, request.into_inner()).await {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => e.error_response(),
    }
}

pub async fn withdraw_funds(pool: web::Data<PgPool>, auth_user: AuthenticatedUser, request: web::Json<WithdrawRequest>) -> impl Responder {
    match WalletService::withdraw(pool.get_ref(), auth_user.user_id, request.into_inner()).await {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => e.error_response(),
    }
}

pub async fn transfer_funds(pool: web::Data<PgPool>, auth_user: AuthenticatedUser, request: web::Json<TransferRequest>) -> impl Responder {
    match WalletService::transfer(pool.get_ref(), auth_user.user_id, request.into_inner()).await {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => e.error_response(),
    }
}

