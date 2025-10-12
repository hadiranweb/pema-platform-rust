use actix_web::{web, HttpResponse, Responder};
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pages")
            .route("", web::post().to(handlers::create_page))
            .route("/{id}", web::get().to(handlers::get_page))
            .route("/{id}", web::put().to(handlers::update_page))
            .route("/{id}", web::delete().to(handlers::delete_page))
    );
}

