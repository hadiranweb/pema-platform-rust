use sqlx::PgPool;
use uuid::Uuid;

use models::review::{CreateReview, UpdateReview, Review};

pub async fn create_review(pool: &PgPool, create_review: CreateReview) -> Result<Review, String> {
    sqlx::query_as::<_, Review>(
        "INSERT INTO reviews (id, product_id, user_id, rating, comment) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(create_review.product_id)
    .bind(create_review.user_id)
    .bind(create_review.rating)
    .bind(create_review.comment)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to create review: {}", e))
}

pub async fn find_review_by_id(pool: &PgPool, review_id: Uuid) -> Result<Review, String> {
    sqlx::query_as::<_, Review>("SELECT * FROM reviews WHERE id = $1")
        .bind(review_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch review by ID: {}", e))?
        .ok_or_else(|| "Review not found".to_string())
}

pub async fn find_reviews_by_product_id(pool: &PgPool, product_id: Uuid) -> Result<Vec<Review>, String> {
    sqlx::query_as::<_, Review>("SELECT * FROM reviews WHERE product_id = $1 ORDER BY created_at DESC")
        .bind(product_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch reviews for product: {}", e))
}

pub async fn find_reviews_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Review>, String> {
    sqlx::query_as::<_, Review>("SELECT * FROM reviews WHERE user_id = $1 ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch reviews by user: {}", e))
}

pub async fn update_review(pool: &PgPool, review_id: Uuid, update_review: UpdateReview) -> Result<Review, String> {
    sqlx::query_as::<_, Review>(
        "UPDATE reviews SET rating = COALESCE($1, rating), comment = COALESCE($2, comment), updated_at = NOW() WHERE id = $3 RETURNING *"
    )
    .bind(update_review.rating)
    .bind(update_review.comment)
    .bind(review_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to update review: {}", e))
}

pub async fn delete_review(pool: &PgPool, review_id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM reviews WHERE id = $1")
        .bind(review_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete review: {}", e))?;
    Ok(())
}

