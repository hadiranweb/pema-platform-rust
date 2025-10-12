
use actix_web::web;

use crate::modules::orders::handlers::{create_order, delete_order, get_all_orders, get_order_by_id, get_user_orders, update_order};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("", web::get().to(get_all_orders))
            .route("", web::post().to(create_order))
            .route("/my", web::get().to(get_user_orders))
            .route("/{id}", web::get().to(get_order_by_id))
            .route("/{id}", web::put().to(update_order))
            .route("/{id}", web::delete().to(delete_order)),
    );
}

