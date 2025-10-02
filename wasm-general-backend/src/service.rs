use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use models::{notification::{Notification, CreateNotification, UpdateNotification}, pagination::{Pagination, PaginatedResponse}};
use crate::error::ServiceError;

pub async fn create_notification(pool: &PgPool, new_notification: CreateNotification) -> Result<Notification, ServiceError> {
    let notification = sqlx::query_as::<_, Notification>(
        "INSERT INTO notifications (user_id, message, notification_type, is_read, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(new_notification.user_id)
    .bind(new_notification.message)
    .bind(new_notification.notification_type)
    .bind(false)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(notification)
}

pub async fn get_notifications(pool: &PgPool, pagination: Pagination) -> Result<PaginatedResponse<Notification>, ServiceError> {
    let offset = (pagination.page - 1) * pagination.limit;

    let total_items: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM notifications")
        .fetch_one(pool)
        .await?;

    let notifications = sqlx::query_as::<_, Notification>(
        "SELECT * FROM notifications ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(pagination.limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    let total_pages = (total_items as f64 / pagination.limit as f64).ceil() as u32;

    Ok(PaginatedResponse {
        items: notifications,
        total_items: total_items as u32,
        current_page: pagination.page,
        total_pages,
        limit: pagination.limit,
    })
}

pub async fn get_notification_by_id(pool: &PgPool, notification_id: Uuid) -> Result<Notification, ServiceError> {
    let notification = sqlx::query_as::<_, Notification>(
        "SELECT * FROM notifications WHERE id = $1"
    )
    .bind(notification_id)
    .fetch_one(pool)
    .await?;

    Ok(notification)
}

pub async fn update_notification(pool: &PgPool, notification_id: Uuid, updated_notification: UpdateNotification) -> Result<Notification, ServiceError> {
    let existing_notification = get_notification_by_id(pool, notification_id).await?;

    let notification = sqlx::query_as::<_, Notification>(
        "UPDATE notifications SET message = $1, notification_type = $2, is_read = $3, updated_at = $4 WHERE id = $5 RETURNING *"
    )
    .bind(updated_notification.message.unwrap_or(existing_notification.message))
    .bind(updated_notification.notification_type.unwrap_or(existing_notification.notification_type))
    .bind(updated_notification.is_read.unwrap_or(existing_notification.is_read))
    .bind(Utc::now())
    .bind(notification_id)
    .fetch_one(pool)
    .await?;

    Ok(notification)
}

pub async fn delete_notification(pool: &PgPool, notification_id: Uuid) -> Result<(), ServiceError> {
    sqlx::query("DELETE FROM notifications WHERE id = $1")
        .bind(notification_id)
        .execute(pool)
        .await?;

    Ok(())
}

