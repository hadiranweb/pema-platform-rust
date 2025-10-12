use actix_web::{web, HttpResponse, Responder};
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shipping")
            .route("", web::post().to(handlers::create_shipping))
            .route("/{id}", web::get().to(handlers::get_shipping))
            .route("/{id}/status", web::put().to(handlers::update_shipping_status))
    );
}

