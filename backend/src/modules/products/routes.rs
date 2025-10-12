
use actix_web::web;

use crate::modules::products::handlers::{create_product, delete_product, get_all_products, get_product_by_id, update_product};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(get_all_products))
            .route("", web::post().to(create_product))
            .route("/{id}", web::get().to(get_product_by_id))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}", web::delete().to(delete_product)),
    );
}

