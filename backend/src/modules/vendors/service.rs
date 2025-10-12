
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::models::vendor::{CreateVendor, UpdateVendor, Vendor};
use crate::modules::vendors::repository;

pub struct VendorService;

impl VendorService {
    pub async fn create_vendor(pool: &PgPool, create_vendor: CreateVendor) -> Result<Vendor, String> {
        repository::create_vendor(pool, create_vendor).await
    }

    pub async fn get_vendor_by_id(pool: &PgPool, vendor_id: Uuid) -> Result<Vendor, String> {
        repository::find_vendor_by_id(pool, vendor_id).await
    }

    pub async fn get_all_vendors(pool: &PgPool) -> Result<Vec<Vendor>, String> {
        repository::find_all_vendors(pool).await
    }

    pub async fn update_vendor(pool: &PgPool, vendor_id: Uuid, update_vendor: UpdateVendor) -> Result<Vendor, String> {
        repository::update_vendor(pool, vendor_id, update_vendor).await
    }

    pub async fn delete_vendor(pool: &PgPool, vendor_id: Uuid) -> Result<(), String> {
        repository::delete_vendor(pool, vendor_id).await
    }
}

