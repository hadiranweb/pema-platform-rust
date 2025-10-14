use sqlx::PgPool;
use crate::error::ServiceError;
use super::repository;

pub async fn create_page(pool: &PgPool, title: String, slug: String, content: String, is_published: bool) -> Result<repository::Page, ServiceError> {
    repository::create_page(pool, title, slug, content, is_published)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}

pub async fn get_page_details(pool: &PgPool, page_id: i32) -> Result<repository::Page, ServiceError> {
    repository::get_page_by_id(pool, page_id)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ServiceError::NotFound("Page not found".to_string()))
}

pub async fn update_page(pool: &PgPool, page_id: i32, title: Option<String>, slug: Option<String>, content: Option<String>, is_published: Option<bool>) -> Result<repository::Page, ServiceError> {
    repository::update_page(pool, page_id, title, slug, content, is_published)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ServiceError::NotFound("Page not found".to_string()))
}

pub async fn delete_page(pool: &PgPool, page_id: i32) -> Result<repository::Page, ServiceError> {
    repository::delete_page(pool, page_id)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ServiceError::NotFound("Page not found".to_string()))
}

pub async fn get_all_pages(pool: &PgPool) -> Result<Vec<repository::Page>, ServiceError> {
    repository::get_all_pages(pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}

pub async fn get_published_pages(pool: &PgPool) -> Result<Vec<repository::Page>, ServiceError> {
    repository::get_published_pages(pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}

