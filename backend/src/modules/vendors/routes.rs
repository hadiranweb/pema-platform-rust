
use actix_web::web;

use crate::modules::vendors::handlers::{create_vendor, delete_vendor, get_all_vendors, get_vendor_by_id, update_vendor};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/vendors")
            .route("", web::get().to(get_all_vendors))
            .route("", web::post().to(create_vendor))
            .route("/{id}", web::get().to(get_vendor_by_id))
            .route("/{id}", web::put().to(update_vendor))
            .route("/{id}", web::delete().to(delete_vendor)),
    );
}

