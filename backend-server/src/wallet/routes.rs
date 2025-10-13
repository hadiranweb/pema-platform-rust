// Temporary simplified routes without authentication middleware
// TODO: Re-enable authentication when middleware issues are resolved

use actix_web::web;
use crate::wallet::handlers;

pub fn wallet_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/wallet")
            // Health check
            .route("/health", web::get().to(handlers::wallet_health_handler))
            
            // Basic wallet routes (without authentication for now)
            .route("/", web::post().to(handlers::create_wallet_handler))
            .route("/user/{user_id}", web::get().to(handlers::get_wallets_by_user_id_handler))
            .route("/{wallet_id}", web::get().to(handlers::get_wallet_by_id_handler))
            
            // Transaction routes
            .route("/transactions", web::post().to(handlers::create_transaction_handler))
            .route("/{wallet_id}/transactions", web::get().to(handlers::get_transactions_by_wallet_id_handler))
            .route("/transactions/{transaction_id}", web::get().to(handlers::get_transaction_by_id_handler))
            
            // Admin routes (temporarily without authentication)
            .service(
                web::scope("/admin")
                    .route("/{wallet_id}/status", web::put().to(handlers::update_wallet_status_handler))
                    .route("/transactions/{transaction_id}/status", web::put().to(handlers::update_transaction_status_handler))
                    .route("/purchase-flows", web::post().to(handlers::create_purchase_flow_handler))
                    .route("/purchase-flows/{flow_id}", web::get().to(handlers::get_purchase_flow_by_id_handler))
                    .route("/purchase-flows/{flow_id}/status", web::put().to(handlers::update_purchase_flow_status_handler))
                    .route("/refund-requests", web::post().to(handlers::create_refund_request_handler))
                    .route("/refund-requests/user/{user_id}", web::get().to(handlers::get_refund_requests_by_user_id_handler))
                    .route("/refund-requests/{request_id}/status", web::put().to(handlers::update_refund_request_status_handler))
                    .route("/admin-actions", web::post().to(handlers::record_admin_action_handler))
                    .route("/admin-actions/admin/{admin_id}", web::get().to(handlers::get_admin_actions_by_admin_id_handler))
                    .route("/admin-actions/target/{target_id}", web::get().to(handlers::get_admin_actions_by_target_id_handler))
            )
    );
}