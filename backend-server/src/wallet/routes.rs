
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::auth::middleware::{validate_jwt_from_bearer, validate_admin_jwt_from_bearer};
use crate::wallet::handlers;

pub fn wallet_routes(cfg: &mut web::ServiceConfig) {
    let auth_middleware = HttpAuthentication::bearer(validate_jwt_from_bearer);
    let admin_auth_middleware = HttpAuthentication::bearer(validate_admin_jwt_from_bearer);

    cfg.service(
        web::scope("/wallet")
            // Authenticated user routes
            .service(
                web::scope("/")
                    .wrap(auth_middleware.clone())
                    .route("/", web::post().to(handlers::create_wallet_handler))
                    .route("/user/{user_id}", web::get().to(handlers::get_wallet_by_user_id_handler))
                    .route("/{wallet_id}", web::get().to(handlers::get_wallet_by_id_handler))
                    .route("/transactions", web::post().to(handlers::create_transaction_handler))
                    .route("/transactions/{transaction_id}", web::get().to(handlers::get_transaction_by_id_handler))
                    .route("/{wallet_id}/transactions", web::get().to(handlers::get_transactions_by_wallet_id_handler))
                    .route("/purchase-flows", web::post().to(handlers::create_purchase_flow_handler))
                    .route("/purchase-flows/{flow_id}", web::get().to(handlers::get_purchase_flow_by_id_handler))
                    .route("/refund-requests", web::post().to(handlers::create_refund_request_handler))
                    .route("/refund-requests/{request_id}", web::get().to(handlers::get_refund_request_by_id_handler))
            )

            // Admin routes (protected by admin_auth_middleware)
            .service(
                web::scope("/admin")
                    .wrap(admin_auth_middleware.clone())
                    .route("/{wallet_id}/status", web::put().to(handlers::update_wallet_status_handler))
                    .route("/transactions/{transaction_id}/status", web::put().to(handlers::update_transaction_status_handler))
                    .route("/purchase-flows/{flow_id}/status", web::put().to(handlers::update_purchase_flow_status_handler))
                    .route("/refund-requests/{request_id}/status", web::put().to(handlers::update_refund_request_status_handler))
                    .route("/admin-actions", web::post().to(handlers::record_admin_action_handler))
                    .route("/admin-actions/admin/{admin_id}", web::get().to(handlers::get_admin_actions_by_admin_id_handler))
                    .route("/admin-actions/target/{target_id}", web::get().to(handlers::get_admin_actions_by_target_id_handler))
            )
    );
}

