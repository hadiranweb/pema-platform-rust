use sqlx::PgPool;
use uuid::Uuid;
use models::vendor::{CreateVendor, UpdateVendor, Vendor};
use crate::modules::vendors::repository;
use crate::error::ServiceError;

pub struct VendorService;

impl VendorService {
    pub async fn create_vendor(pool: &PgPool, create_vendor: CreateVendor) -> Result<Vendor, ServiceError> {
        repository::create_vendor(pool, create_vendor).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn get_vendor_by_id(pool: &PgPool, vendor_id: Uuid) -> Result<Vendor, ServiceError> {
        repository::find_vendor_by_id(pool, vendor_id).await.map_err(ServiceError::NotFound)
    }

    pub async fn get_all_vendors(pool: &PgPool) -> Result<Vec<Vendor>, ServiceError> {
        repository::find_all_vendors(pool).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn update_vendor(pool: &PgPool, vendor_id: Uuid, update_vendor: UpdateVendor) -> Result<Vendor, ServiceError> {
        repository::update_vendor(pool, vendor_id, update_vendor).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn delete_vendor(pool: &PgPool, vendor_id: Uuid) -> Result<(), ServiceError> {
        repository::delete_vendor(pool, vendor_id).await.map_err(ServiceError::DatabaseError)
    }
}

