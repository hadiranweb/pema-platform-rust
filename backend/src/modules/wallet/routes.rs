
use actix_web::web;

use crate::modules::auth::middleware::JwtMiddleware;
use crate::modules::wallet::handlers::{deposit_funds, get_my_wallet, get_wallet_transactions, transfer_funds, withdraw_funds};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/wallet")
            .wrap(JwtMiddleware) // Protect all wallet routes
            .route("/me", web::get().to(get_my_wallet))
            .route("/me/transactions", web::get().to(get_wallet_transactions))
            .route("/deposit", web::post().to(deposit_funds))
            .route("/withdraw", web::post().to(withdraw_funds))
            .route("/transfer", web::post().to(transfer_funds)),
    );
}

