use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use models::{inventory::{InventoryItem, CreateInventoryItem, UpdateInventoryItem}, pagination::{Pagination, PaginatedResponse}};
use crate::error::ServiceError;

pub async fn create_inventory_item(pool: &PgPool, new_item: CreateInventoryItem) -> Result<InventoryItem, ServiceError> {
    let item = sqlx::query_as::<_, InventoryItem>(
        "INSERT INTO inventory_items (product_id, quantity, location, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(new_item.product_id)
    .bind(new_item.quantity)
    .bind(new_item.location)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(item)
}

pub async fn get_inventory_items(pool: &PgPool, pagination: Pagination) -> Result<PaginatedResponse<InventoryItem>, ServiceError> {
    let offset = (pagination.page - 1) * pagination.limit;

    let total_items: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM inventory_items")
        .fetch_one(pool)
        .await?;

    let items = sqlx::query_as::<_, InventoryItem>(
        "SELECT * FROM inventory_items ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(pagination.limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    let total_pages = (total_items as f64 / pagination.limit as f64).ceil() as u32;

    Ok(PaginatedResponse {
        items,
        total_items: total_items as u32,
        current_page: pagination.page,
        total_pages,
        limit: pagination.limit,
    })
}

pub async fn get_inventory_item_by_id(pool: &PgPool, item_id: Uuid) -> Result<InventoryItem, ServiceError> {
    let item = sqlx::query_as::<_, InventoryItem>(
        "SELECT * FROM inventory_items WHERE id = $1"
    )
    .bind(item_id)
    .fetch_one(pool)
    .await?;

    Ok(item)
}

pub async fn update_inventory_item(pool: &PgPool, item_id: Uuid, updated_item: UpdateInventoryItem) -> Result<InventoryItem, ServiceError> {
    let existing_item = get_inventory_item_by_id(pool, item_id).await?;

    let item = sqlx::query_as::<_, InventoryItem>(
        "UPDATE inventory_items SET quantity = $1, location = $2, updated_at = $3 WHERE id = $4 RETURNING *"
    )
    .bind(updated_item.quantity.unwrap_or(existing_item.quantity))
    .bind(updated_item.location.unwrap_or(existing_item.location))
    .bind(Utc::now())
    .bind(item_id)
    .fetch_one(pool)
    .await?;

    Ok(item)
}

pub async fn delete_inventory_item(pool: &PgPool, item_id: Uuid) -> Result<(), ServiceError> {
    sqlx::query("DELETE FROM inventory_items WHERE id = $1")
        .bind(item_id)
        .execute(pool)
        .await?;

    Ok(())
}

