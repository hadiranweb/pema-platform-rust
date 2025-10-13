
use sqlx::PgPool;
use uuid::Uuid;

use models::product::{CreateProduct, Product, UpdateProduct};
use crate::modules::products::repository;
use crate::core::plugins::manager::PluginManager;
use pema_plugin_sdk::interface::PluginHookType;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::error::ServiceError;

pub struct ProductService;

impl ProductService {
    pub async fn create_product(pool: &PgPool, create_product: CreateProduct, vendor_id: Uuid) -> Result<Product, ServiceError> {
        repository::create_product(pool, create_product, vendor_id).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn get_product_by_id(pool: &PgPool, product_id: Uuid, plugin_manager: Arc<PluginManager>) -> Result<Product, ServiceError> {
        let product = repository::find_product_by_id(pool, product_id).await.map_err(ServiceError::NotFound)?;

        // Execute OnProductViewed plugin hook
        let _ = plugin_manager.execute_hook::<String, ()>(PluginHookType::OnProductViewed, product_id.to_string()).await.map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

        Ok(product)
    }

    pub async fn get_all_products(pool: &PgPool) -> Result<Vec<Product>, ServiceError> {
        repository::find_all_products(pool).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn update_product(pool: &PgPool, product_id: Uuid, update_product: UpdateProduct, vendor_id: Uuid) -> Result<Product, ServiceError> {
        // First, check if the product exists and belongs to the vendor
        let existing_product = repository::find_product_by_id(pool, product_id).await.map_err(ServiceError::NotFound)?;
        if existing_product.vendor_id != vendor_id {
            return Err(ServiceError::Forbidden);
        }
        repository::update_product(pool, product_id, update_product).await.map_err(ServiceError::DatabaseError)
    }

    pub async fn delete_product(pool: &PgPool, product_id: Uuid, vendor_id: Uuid) -> Result<(), ServiceError> {
        // First, check if the product exists and belongs to the vendor
        let existing_product = repository::find_product_by_id(pool, product_id).await.map_err(ServiceError::NotFound)?;
        if existing_product.vendor_id != vendor_id {
            return Err(ServiceError::Forbidden);
        }
        repository::delete_product(pool, product_id).await.map_err(ServiceError::DatabaseError)
    }
}

