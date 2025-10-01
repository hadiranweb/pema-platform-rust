use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use models::{order::{Order, CreateOrder, UpdateOrder}, pagination::{Pagination, PaginatedResponse}};
use crate::error::ServiceError;

pub async fn create_order(pool: &PgPool, new_order: CreateOrder) -> Result<Order, ServiceError> {
    let order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (user_id, status, total_amount, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(new_order.user_id)
    .bind("pending".to_string())
    .bind(new_order.total_amount)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(order)
}

pub async fn get_orders(pool: &PgPool, pagination: Pagination) -> Result<PaginatedResponse<Order>, ServiceError> {
    let offset = (pagination.page - 1) * pagination.limit;

    let total_items: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM orders")
        .fetch_one(pool)
        .await?;

    let orders = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(pagination.limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    let total_pages = (total_items as f64 / pagination.limit as f64).ceil() as u32;

    Ok(PaginatedResponse {
        items: orders,
        total_items: total_items as u32,
        current_page: pagination.page,
        total_pages,
        limit: pagination.limit,
    })
}

pub async fn get_order_by_id(pool: &PgPool, order_id: Uuid) -> Result<Order, ServiceError> {
    let order = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE id = $1"
    )
    .bind(order_id)
    .fetch_one(pool)
    .await?;

    Ok(order)
}

pub async fn update_order(pool: &PgPool, order_id: Uuid, updated_order: UpdateOrder) -> Result<Order, ServiceError> {
    let existing_order = get_order_by_id(pool, order_id).await?;

    let order = sqlx::query_as::<_, Order>(
        "UPDATE orders SET status = $1, total_amount = $2, updated_at = $3 WHERE id = $4 RETURNING *"
    )
    .bind(updated_order.status.unwrap_or(existing_order.status))
    .bind(updated_order.total_amount.unwrap_or(existing_order.total_amount))
    .bind(Utc::now())
    .bind(order_id)
    .fetch_one(pool)
    .await?;

    Ok(order)
}

pub async fn delete_order(pool: &PgPool, order_id: Uuid) -> Result<(), ServiceError> {
    sqlx::query("DELETE FROM orders WHERE id = $1")
        .bind(order_id)
        .execute(pool)
        .await?;

    Ok(())
}

