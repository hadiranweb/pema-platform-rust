
use actix_web::web;

use crate::modules::auth::middleware::JwtMiddleware;
use crate::modules::reviews::handlers::{create_product_review, delete_review, get_my_reviews, get_review_by_id, get_reviews_for_product, update_review};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reviews")
            .wrap(JwtMiddleware) // Protect all review routes
            .route("", web::post().to(create_product_review))
            .route("/product/{id}", web::get().to(get_reviews_for_product))
            .route("/me", web::get().to(get_my_reviews))
            .route("/{id}", web::get().to(get_review_by_id))
            .route("/{id}", web::put().to(update_review))
            .route("/{id}", web::delete().to(delete_review)),
    );
}

