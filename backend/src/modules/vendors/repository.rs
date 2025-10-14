
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::shared::models::vendor::{CreateVendor, UpdateVendor, Vendor};

pub async fn create_vendor(pool: &PgPool, create_vendor: CreateVendor) -> Result<Vendor, String> {
    let vendor = sqlx::query_as::<_, Vendor>(
        "INSERT INTO vendors (id, name, description) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(create_vendor.name)
    .bind(create_vendor.description)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to create vendor: {}", e))?;

    Ok(vendor)
}

pub async fn find_vendor_by_id(pool: &PgPool, vendor_id: Uuid) -> Result<Vendor, String> {
    let vendor = sqlx::query_as::<_, Vendor>(
        "SELECT * FROM vendors WHERE id = $1"
    )
    .bind(vendor_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to fetch vendor by ID: {}", e))?
    .ok_or_else(|| "Vendor not found".to_string())?;

    Ok(vendor)
}

pub async fn find_all_vendors(pool: &PgPool) -> Result<Vec<Vendor>, String> {
    let vendors = sqlx::query_as::<_, Vendor>(
        "SELECT * FROM vendors"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch all vendors: {}", e))?;

    Ok(vendors)
}

pub async fn update_vendor(pool: &PgPool, vendor_id: Uuid, update_vendor: UpdateVendor) -> Result<Vendor, String> {
    let vendor = sqlx::query_as::<_, Vendor>(
        "UPDATE vendors SET name = COALESCE($1, name), description = COALESCE($2, description), updated_at = NOW() WHERE id = $3 RETURNING *"
    )
    .bind(update_vendor.name)
    .bind(update_vendor.description)
    .bind(vendor_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to update vendor: {}", e))?;

    Ok(vendor)
}

pub async fn delete_vendor(pool: &PgPool, vendor_id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM vendors WHERE id = $1")
        .bind(vendor_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete vendor: {}", e))?;

    Ok(())
}

