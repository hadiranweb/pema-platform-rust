use sqlx::PgPool;
use rand::thread_rng;

use uuid::Uuid;
use chrono::Utc;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use models::{user::{User, CreateUser, UpdateUser}, pagination::{Pagination, PaginatedResponse}};
use crate::error::ServiceError;

pub async fn create_user(pool: &PgPool, new_user: CreateUser) -> Result<User, ServiceError> {
    let mut rng = thread_rng();
    let salt = argon2::password_hash::SaltString::generate(&mut rng);
    let hashed_password = Argon2::default()
        .hash_password(new_user.password.as_bytes(), &salt)
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to hash password: {}", e).to_string()))?
        .to_string();

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(new_user.username)
    .bind(new_user.email)
    .bind(hashed_password)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_users(pool: &PgPool, pagination: Pagination) -> Result<PaginatedResponse<User>, ServiceError> {
    let offset = (pagination.page - 1) * pagination.limit;

    let total_items: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(pagination.limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    let total_pages = (total_items as f64 / pagination.limit as f64).ceil() as u32;

    Ok(PaginatedResponse {
        items: users,
        total_items: total_items as u32,
        current_page: pagination.page,
        total_pages,
        limit: pagination.limit,
    })
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User, ServiceError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn update_user(pool: &PgPool, user_id: Uuid, updated_user: UpdateUser) -> Result<User, ServiceError> {
    let existing_user = get_user_by_id(pool, user_id).await?;

    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET username = $1, email = $2, updated_at = $3 WHERE id = $4 RETURNING *"
    )
    .bind(updated_user.username.unwrap_or(existing_user.username))
    .bind(updated_user.email.unwrap_or(existing_user.email))
    .bind(Utc::now())
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<(), ServiceError> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn verify_password(hashed_password: &str, password: &str) -> Result<bool, ServiceError> {
    let parsed_hash = argon2::password_hash::PasswordHash::new(hashed_password)
        .map_err(|e| ServiceError::InternalServerError(format!("Failed to parse hash: {}", e)))?;

    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

