use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use models::{vendor::{Vendor, CreateVendor, UpdateVendor}, pagination::{Pagination, PaginatedResponse}};
use crate::error::ServiceError;

pub async fn create_vendor(pool: &PgPool, new_vendor: CreateVendor) -> Result<Vendor, ServiceError> {
    let vendor = sqlx::query_as::<_, Vendor>(
        "INSERT INTO vendors (name, contact_person, email, phone, address, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(new_vendor.name)
    .bind(new_vendor.contact_person)
    .bind(new_vendor.email)
    .bind(new_vendor.phone)
    .bind(new_vendor.address)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(vendor)
}

pub async fn get_vendors(pool: &PgPool, pagination: Pagination) -> Result<PaginatedResponse<Vendor>, ServiceError> {
    let offset = (pagination.page - 1) * pagination.limit;

    let total_items: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM vendors")
        .fetch_one(pool)
        .await?;

    let vendors = sqlx::query_as::<_, Vendor>(
        "SELECT * FROM vendors ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(pagination.limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    let total_pages = (total_items as f64 / pagination.limit as f64).ceil() as u32;

    Ok(PaginatedResponse {
        items: vendors,
        total_items: total_items as u32,
        current_page: pagination.page,
        total_pages,
        limit: pagination.limit,
    })
}

pub async fn get_vendor_by_id(pool: &PgPool, vendor_id: Uuid) -> Result<Vendor, ServiceError> {
    let vendor = sqlx::query_as::<_, Vendor>(
        "SELECT * FROM vendors WHERE id = $1"
    )
    .bind(vendor_id)
    .fetch_one(pool)
    .await?;

    Ok(vendor)
}

pub async fn update_vendor(pool: &PgPool, vendor_id: Uuid, updated_vendor: UpdateVendor) -> Result<Vendor, ServiceError> {
    let existing_vendor = get_vendor_by_id(pool, vendor_id).await?;

    let vendor = sqlx::query_as::<_, Vendor>(
        "UPDATE vendors SET name = $1, contact_person = $2, email = $3, phone = $4, address = $5, updated_at = $6 WHERE id = $7 RETURNING *"
    )
    .bind(updated_vendor.name.unwrap_or(existing_vendor.name))
    .bind(updated_vendor.contact_person.unwrap_or(existing_vendor.contact_person))
    .bind(updated_vendor.email.unwrap_or(existing_vendor.email))
    .bind(updated_vendor.phone.unwrap_or(existing_vendor.phone))
    .bind(updated_vendor.address.unwrap_or(existing_vendor.address))
    .bind(Utc::now())
    .bind(vendor_id)
    .fetch_one(pool)
    .await?;

    Ok(vendor)
}

pub async fn delete_vendor(pool: &PgPool, vendor_id: Uuid) -> Result<(), ServiceError> {
    sqlx::query("DELETE FROM vendors WHERE id = $1")
        .bind(vendor_id)
        .execute(pool)
        .await?;

    Ok(())
}

