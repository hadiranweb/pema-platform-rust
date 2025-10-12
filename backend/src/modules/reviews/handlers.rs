
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::auth::middleware::AuthenticatedUser;
use crate::modules::reviews::dto::{CreateReview, UpdateReview};
use crate::modules::reviews::service::ReviewService;

pub async fn create_product_review(pool: web::Data<PgPool>, auth_user: AuthenticatedUser, new_review: web::Json<CreateReview>) -> impl Responder {
    let mut review_data = new_review.into_inner();
    review_data.user_id = auth_user.user_id;
    match ReviewService::create_review(pool.get_ref(), review_data).await {
        Ok(review) => HttpResponse::Created().json(review),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_review_by_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let review_id = path.into_inner();
    match ReviewService::get_review_by_id(pool.get_ref(), review_id).await {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

pub async fn get_reviews_for_product(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let product_id = path.into_inner();
    match ReviewService::get_reviews_by_product_id(pool.get_ref(), product_id).await {
        Ok(reviews) => HttpResponse::Ok().json(reviews),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_my_reviews(pool: web::Data<PgPool>, auth_user: AuthenticatedUser) -> impl Responder {
    match ReviewService::get_reviews_by_user_id(pool.get_ref(), auth_user.user_id).await {
        Ok(reviews) => HttpResponse::Ok().json(reviews),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn update_review(pool: web::Data<PgPool>, path: web::Path<Uuid>, auth_user: AuthenticatedUser, update_review: web::Json<UpdateReview>) -> impl Responder {
    let review_id = path.into_inner();
    // Add logic to ensure the user updating the review is the owner of the review
    match ReviewService::update_review(pool.get_ref(), review_id, update_review.into_inner()).await {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn delete_review(pool: web::Data<PgPool>, path: web::Path<Uuid>, auth_user: AuthenticatedUser) -> impl Responder {
    let review_id = path.into_inner();
    // Add logic to ensure the user deleting the review is the owner of the review or an admin
    match ReviewService::delete_review(pool.get_ref(), review_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

