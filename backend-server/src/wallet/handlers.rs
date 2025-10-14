
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::wallet::models::{WalletResponse, UpdateWalletRequest, CreateTransactionRequest, TransactionResponse, UpdateTransactionStatusRequest, CreatePurchaseFlowRequest, PurchaseFlowResponse, UpdatePurchaseFlowStatusRequest, CreateRefundRequest, RefundRequestResponse, UpdateRefundRequestStatus, CreateAdminActionRequest, AdminActionResponse, WalletStatus, TransactionType, TransactionStatus, PurchaseFlowStatus, RefundStatus, AdminActionType};
use crate::auth::middleware::AuthenticatedUser;
use crate::wallet::service::WalletService;
use crate::wallet::errors::WalletError;

// --- Wallet Handlers ---

pub async fn create_wallet_handler(pool: web::Data<PgPool>, auth_user: AuthenticatedUser) -> Result<HttpResponse, WalletError> {
    let service = WalletService::new(pool.get_ref().clone());
    let user_id = uuid::Uuid::parse_str(&auth_user.claims.sub)
        .map_err(|_| WalletError::InvalidInput("Invalid user ID format".to_string()))?;
    let wallet = service.create_wallet(user_id).await?;
    Ok(HttpResponse::Created().json(WalletResponse {
        id: wallet.id,
        user_id: wallet.user_id,
        balance: wallet.balance,
        currency: wallet.currency,
        status: wallet.status,
        created_at: wallet.created_at,
        updated_at: wallet.updated_at,
    }))
}

pub async fn get_wallet_by_user_id_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> Result<HttpResponse, WalletError> {
    let user_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let wallet = service.get_wallet_by_user_id(user_id).await?;
    Ok(HttpResponse::Ok().json(WalletResponse {
        id: wallet.id,
        user_id: wallet.user_id,
        balance: wallet.balance,
        currency: wallet.currency,
        status: wallet.status,
        created_at: wallet.created_at,
        updated_at: wallet.updated_at,
    }))
}

pub async fn get_wallet_by_id_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> Result<HttpResponse, WalletError> {
    let wallet_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let wallet = service.get_wallet_by_id(wallet_id).await?;
    Ok(HttpResponse::Ok().json(WalletResponse {
        id: wallet.id,
        user_id: wallet.user_id,
        balance: wallet.balance,
        currency: wallet.currency,
        status: wallet.status,
        created_at: wallet.created_at,
        updated_at: wallet.updated_at,
    }))
}

pub async fn update_wallet_status_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>, req: web::Json<UpdateWalletRequest>) -> Result<HttpResponse, WalletError> {
    let wallet_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let status_str = req.status.clone().ok_or(WalletError::InvalidInput("Wallet status is required".to_string()))?;
    let status = WalletStatus::from_str(&status_str).map_err(|_| WalletError::InvalidInput("Invalid wallet status".to_string()))?;
    let wallet = service.update_wallet_status(wallet_id, status).await?;
    Ok(HttpResponse::Ok().json(WalletResponse {
        id: wallet.id,
        user_id: wallet.user_id,
        balance: wallet.balance,
        currency: wallet.currency,
        status: wallet.status,
        created_at: wallet.created_at,
        updated_at: wallet.updated_at,
    }))
}

// --- Transaction Handlers ---

pub async fn create_transaction_handler(pool: web::Data<PgPool>, req: web::Json<CreateTransactionRequest>) -> Result<HttpResponse, WalletError> {
    let service = WalletService::new(pool.get_ref().clone());
    let transaction_type = TransactionType::from_str(&req.transaction_type).map_err(|_| WalletError::InvalidInput("Invalid transaction type".to_string()))?;
    let transaction = service.create_transaction(
        req.wallet_id,
        transaction_type,
        req.amount,
        req.description.clone(),
        req.reference_id,
    ).await?;
    Ok(HttpResponse::Created().json(TransactionResponse {
        id: transaction.id,
        wallet_id: transaction.wallet_id,
        transaction_type: transaction.transaction_type,
        amount: transaction.amount,
        status: transaction.status,
        description: transaction.description,
        reference_id: transaction.reference_id,
        created_at: transaction.created_at,
        updated_at: transaction.updated_at,
    }))
}

pub async fn get_transactions_by_wallet_id_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> Result<HttpResponse, WalletError> {
    let wallet_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let transactions = service.get_transactions_by_wallet_id(wallet_id).await?;
    Ok(HttpResponse::Ok().json(transactions.into_iter().map(|t| TransactionResponse {
        id: t.id,
        wallet_id: t.wallet_id,
        transaction_type: t.transaction_type,
        amount: t.amount,
        status: t.status,
        description: t.description,
        reference_id: t.reference_id,
        created_at: t.created_at,
        updated_at: t.updated_at,
    }).collect::<Vec<TransactionResponse>>()))
}

pub async fn get_transaction_by_id_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> Result<HttpResponse, WalletError> {
    let transaction_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let transaction = service.get_transaction_by_id(transaction_id).await?;
    Ok(HttpResponse::Ok().json(TransactionResponse {
        id: transaction.id,
        wallet_id: transaction.wallet_id,
        transaction_type: transaction.transaction_type,
        amount: transaction.amount,
        status: transaction.status,
        description: transaction.description,
        reference_id: transaction.reference_id,
        created_at: transaction.created_at,
        updated_at: transaction.updated_at,
    }))
}

pub async fn update_transaction_status_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>, req: web::Json<UpdateTransactionStatusRequest>) -> Result<HttpResponse, WalletError> {
    let transaction_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let status = TransactionStatus::from_str(&req.status).map_err(|_| WalletError::InvalidInput("Invalid transaction status".to_string()))?;
    let transaction = service.update_transaction_status(transaction_id, status).await?;
    Ok(HttpResponse::Ok().json(TransactionResponse {
        id: transaction.id,
        wallet_id: transaction.wallet_id,
        transaction_type: transaction.transaction_type,
        amount: transaction.amount,
        status: transaction.status,
        description: transaction.description,
        reference_id: transaction.reference_id,
        created_at: transaction.created_at,
        updated_at: transaction.updated_at,
    }))
}

// --- Purchase Flow Handlers ---

pub async fn create_purchase_flow_handler(pool: web::Data<PgPool>, req: web::Json<CreatePurchaseFlowRequest>) -> Result<HttpResponse, WalletError> {
    let service = WalletService::new(pool.get_ref().clone());
    let purchase_flow = service.create_purchase_flow(
        req.user_id,
        req.wallet_id,
        req.amount,
        req.metadata.clone(),
    ).await?;
    Ok(HttpResponse::Created().json(PurchaseFlowResponse {
        id: purchase_flow.id,
        user_id: purchase_flow.user_id,
        wallet_id: purchase_flow.wallet_id,
        amount: purchase_flow.amount,
        status: purchase_flow.status,
        metadata: purchase_flow.metadata,
        created_at: purchase_flow.created_at,
        updated_at: purchase_flow.updated_at,
    }))
}

pub async fn get_purchase_flow_by_id_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> Result<HttpResponse, WalletError> {
    let flow_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let purchase_flow = service.get_purchase_flow_by_id(flow_id).await?;
    Ok(HttpResponse::Ok().json(PurchaseFlowResponse {
        id: purchase_flow.id,
        user_id: purchase_flow.user_id,
        wallet_id: purchase_flow.wallet_id,
        amount: purchase_flow.amount,
        status: purchase_flow.status,
        metadata: purchase_flow.metadata,
        created_at: purchase_flow.created_at,
        updated_at: purchase_flow.updated_at,
    }))
}

pub async fn update_purchase_flow_status_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>, req: web::Json<UpdatePurchaseFlowStatusRequest>) -> Result<HttpResponse, WalletError> {
    let flow_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let status = PurchaseFlowStatus::from_str(&req.status).map_err(|_| WalletError::InvalidInput("Invalid purchase flow status".to_string()))?;
    let purchase_flow = service.update_purchase_flow_status(flow_id, status).await?;
    Ok(HttpResponse::Ok().json(PurchaseFlowResponse {
        id: purchase_flow.id,
        user_id: purchase_flow.user_id,
        wallet_id: purchase_flow.wallet_id,
        amount: purchase_flow.amount,
        status: purchase_flow.status,
        metadata: purchase_flow.metadata,
        created_at: purchase_flow.created_at,
        updated_at: purchase_flow.updated_at,
    }))
}

// --- Refund Request Handlers ---

pub async fn create_refund_request_handler(pool: web::Data<PgPool>, req: web::Json<CreateRefundRequest>) -> Result<HttpResponse, WalletError> {
    let service = WalletService::new(pool.get_ref().clone());
    let refund_request = service.create_refund_request(
        req.transaction_id,
        req.user_id,
        req.amount,
        req.reason.clone(),
    ).await?;
    Ok(HttpResponse::Created().json(RefundRequestResponse {
        id: refund_request.id,
        transaction_id: refund_request.transaction_id,
        user_id: refund_request.user_id,
        amount: refund_request.amount,
        reason: refund_request.reason,
        status: refund_request.status,
        created_at: refund_request.created_at,
        updated_at: refund_request.updated_at,
    }))
}

pub async fn get_refund_request_by_id_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> Result<HttpResponse, WalletError> {
    let request_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let refund_request = service.get_refund_request_by_id(request_id).await?;
    Ok(HttpResponse::Ok().json(RefundRequestResponse {
        id: refund_request.id,
        transaction_id: refund_request.transaction_id,
        user_id: refund_request.user_id,
        amount: refund_request.amount,
        reason: refund_request.reason,
        status: refund_request.status,
        created_at: refund_request.created_at,
        updated_at: refund_request.updated_at,
    }))
}

pub async fn update_refund_request_status_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>, req: web::Json<UpdateRefundRequestStatus>) -> Result<HttpResponse, WalletError> {
    let request_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let status = RefundStatus::from_str(&req.status).map_err(|_| WalletError::InvalidInput("Invalid refund status".to_string()))?;
    let refund_request = service.update_refund_request_status(request_id, status).await?;
    Ok(HttpResponse::Ok().json(RefundRequestResponse {
        id: refund_request.id,
        transaction_id: refund_request.transaction_id,
        user_id: refund_request.user_id,
        amount: refund_request.amount,
        reason: refund_request.reason,
        status: refund_request.status,
        created_at: refund_request.created_at,
        updated_at: refund_request.updated_at,
    }))
}

// --- Admin Action Handlers ---

pub async fn record_admin_action_handler(pool: web::Data<PgPool>, req: web::Json<CreateAdminActionRequest>) -> Result<HttpResponse, WalletError> {
    let service = WalletService::new(pool.get_ref().clone());
    let action_type = AdminActionType::from_str(&req.admin_action_type).map_err(|_| WalletError::InvalidInput("Invalid admin action type".to_string()))?;
    let admin_action = service.record_admin_action(
        req.admin_id,
        action_type,
        req.target_id,
        req.details.clone(),
    ).await?;
    Ok(HttpResponse::Created().json(AdminActionResponse {
        id: admin_action.id,
        admin_id: admin_action.admin_id,
        admin_action_type: admin_action.admin_action_type,
        target_id: admin_action.target_id,
        details: admin_action.details,
        created_at: admin_action.created_at,
    }))
}

pub async fn get_admin_actions_by_admin_id_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> Result<HttpResponse, WalletError> {
    let admin_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let admin_actions = service.get_admin_actions_by_admin_id(admin_id).await?;
    Ok(HttpResponse::Ok().json(admin_actions.into_iter().map(|a| AdminActionResponse {
        id: a.id,
        admin_id: a.admin_id,
        admin_action_type: a.admin_action_type,
        target_id: a.target_id,
        details: a.details,
        created_at: a.created_at,
    }).collect::<Vec<AdminActionResponse>>()))
}

pub async fn get_admin_actions_by_target_id_handler(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> Result<HttpResponse, WalletError> {
    let target_id = path.into_inner();
    let service = WalletService::new(pool.get_ref().clone());
    let admin_actions = service.get_admin_actions_by_target_id(target_id).await?;
    Ok(HttpResponse::Ok().json(admin_actions.into_iter().map(|a| AdminActionResponse {
        id: a.id,
        admin_id: a.admin_id,
        admin_action_type: a.admin_action_type,
        target_id: a.target_id,
        details: a.details,
        created_at: a.created_at,
    }).collect::<Vec<AdminActionResponse>>()))
}

