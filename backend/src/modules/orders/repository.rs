
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::shared::models::order::{CreateOrder, Order, UpdateOrder};

pub async fn create_order(pool: &PgPool, create_order: CreateOrder, user_id: Uuid) -> Result<Order, String> {
    let order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (id, user_id, status, total_amount) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind("pending") // Default status
    .bind(create_order.total_amount)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to create order: {}", e))?;

    Ok(order)
}

pub async fn find_order_by_id(pool: &PgPool, order_id: Uuid) -> Result<Order, String> {
    let order = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE id = $1"
    )
    .bind(order_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to fetch order by ID: {}", e))?
    .ok_or_else(|| "Order not found".to_string())?;

    Ok(order)
}

pub async fn find_all_orders(pool: &PgPool) -> Result<Vec<Order>, String> {
    let orders = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch all orders: {}", e))?;

    Ok(orders)
}

pub async fn find_user_orders(pool: &PgPool, user_id: Uuid) -> Result<Vec<Order>, String> {
    let orders = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch user orders: {}", e))?;

    Ok(orders)
}

pub async fn update_order(pool: &PgPool, order_id: Uuid, update_order: UpdateOrder) -> Result<Order, String> {
    let order = sqlx::query_as::<_, Order>(
        "UPDATE orders SET status = COALESCE($1, status), total_amount = COALESCE($2, total_amount), updated_at = NOW() WHERE id = $3 RETURNING *"
    )
    .bind(update_order.status)
    .bind(update_order.total_amount)
    .bind(order_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to update order: {}", e))?;

    Ok(order)
}

pub async fn delete_order(pool: &PgPool, order_id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM orders WHERE id = $1")
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete order: {}", e))?;

    Ok(())
}

