use sqlx::PgPool;
use uuid::Uuid;
use models::review::{CreateReview, UpdateReview, Review};
use crate::modules::reviews::repository;
use crate::error::ServiceError;

pub struct ReviewService;

impl ReviewService {
    pub async fn create_review(pool: &PgPool, create_review: CreateReview) -> Result<Review, ServiceError> {
        repository::create_review(pool, create_review).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn get_review_by_id(pool: &PgPool, review_id: Uuid) -> Result<Review, ServiceError> {
        repository::find_review_by_id(pool, review_id).await.map_err(ServiceError::NotFound)
    }

    pub async fn get_reviews_by_product_id(pool: &PgPool, product_id: Uuid) -> Result<Vec<Review>, ServiceError> {
        repository::find_reviews_by_product_id(pool, product_id).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn get_reviews_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Review>, ServiceError> {
        repository::find_reviews_by_user_id(pool, user_id).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn update_review(pool: &PgPool, review_id: Uuid, update_review: UpdateReview) -> Result<Review, ServiceError> {
        repository::update_review(pool, review_id, update_review).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn delete_review(pool: &PgPool, review_id: Uuid) -> Result<(), ServiceError> {
        repository::delete_review(pool, review_id).await.map_err(ServiceError::DatabaseError)
    }
}

