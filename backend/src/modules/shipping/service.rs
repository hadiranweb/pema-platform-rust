use sqlx::PgPool;
use uuid::Uuid;
use crate::error::ServiceError;
use super::repository;

pub async fn create_shipping(pool: &PgPool, order_id: Uuid, address: String, city: String, state: String, zip_code: String, country: String) -> Result<repository::Shipping, ServiceError> {
    repository::create_shipping(pool, order_id, address, city, state, zip_code, country)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}

pub async fn get_shipping_details(pool: &PgPool, shipping_id: Uuid) -> Result<repository::Shipping, ServiceError> {
    repository::get_shipping_by_id(pool, shipping_id)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ServiceError::NotFound("Shipping record not found".to_string()))
}

pub async fn update_shipping_status(pool: &PgPool, shipping_id: Uuid, status: &str) -> Result<repository::Shipping, ServiceError> {
    repository::update_shipping_status(pool, shipping_id, status)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ServiceError::NotFound("Shipping record not found".to_string()))
}

