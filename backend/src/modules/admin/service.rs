
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::models::user::User;
use crate::shared::models::product::{Product, CreateProduct, UpdateProduct};
use crate::shared::models::order::{Order, CreateOrder, UpdateOrder};
use crate::shared::models::vendor::{Vendor, CreateVendor, UpdateVendor};
use crate::shared::models::page::{Page, CreatePage, UpdatePage};

use crate::modules::admin::repository;
use crate::modules::admin::dto::{AdminUserUpdateDto, AdminProductUpdateDto, AdminOrderUpdateDto, AdminVendorUpdateDto, AdminPageUpdateDto};

pub struct AdminService;

impl AdminService {
    // User Management
    pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, String> {
        repository::find_all_users(pool).await
    }

    pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User, String> {
        repository::find_user_by_id(pool, user_id).await
    }

    pub async fn update_user(pool: &PgPool, user_id: Uuid, update_dto: AdminUserUpdateDto) -> Result<User, String> {
        repository::update_user(pool, user_id, update_dto).await
    }

    pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<(), String> {
        repository::delete_user(pool, user_id).await
    }

    // Product Management
    pub async fn get_all_products(pool: &PgPool) -> Result<Vec<Product>, String> {
        repository::find_all_products(pool).await
    }

    pub async fn get_product_by_id(pool: &PgPool, product_id: Uuid) -> Result<Product, String> {
        repository::find_product_by_id(pool, product_id).await
    }

    pub async fn create_product(pool: &PgPool, create_product: CreateProduct) -> Result<Product, String> {
        repository::create_product(pool, create_product).await
    }

    pub async fn update_product(pool: &PgPool, product_id: Uuid, update_dto: AdminProductUpdateDto) -> Result<Product, String> {
        repository::update_product(pool, product_id, update_dto).await
    }

    pub async fn delete_product(pool: &PgPool, product_id: Uuid) -> Result<(), String> {
        repository::delete_product(pool, product_id).await
    }

    // Order Management
    pub async fn get_all_orders(pool: &PgPool) -> Result<Vec<Order>, String> {
        repository::find_all_orders(pool).await
    }

    pub async fn get_order_by_id(pool: &PgPool, order_id: Uuid) -> Result<Order, String> {
        repository::find_order_by_id(pool, order_id).await
    }

    pub async fn update_order(pool: &PgPool, order_id: Uuid, update_dto: AdminOrderUpdateDto) -> Result<Order, String> {
        repository::update_order(pool, order_id, update_dto).await
    }

    pub async fn delete_order(pool: &PgPool, order_id: Uuid) -> Result<(), String> {
        repository::delete_order(pool, order_id).await
    }

    // Vendor Management
    pub async fn get_all_vendors(pool: &PgPool) -> Result<Vec<Vendor>, String> {
        repository::find_all_vendors(pool).await
    }

    pub async fn get_vendor_by_id(pool: &PgPool, vendor_id: Uuid) -> Result<Vendor, String> {
        repository::find_vendor_by_id(pool, vendor_id).await
    }

    pub async fn create_vendor(pool: &PgPool, create_vendor: CreateVendor) -> Result<Vendor, String> {
        repository::create_vendor(pool, create_vendor).await
    }

    pub async fn update_vendor(pool: &PgPool, vendor_id: Uuid, update_dto: AdminVendorUpdateDto) -> Result<Vendor, String> {
        repository::update_vendor(pool, vendor_id, update_dto).await
    }

    pub async fn delete_vendor(pool: &PgPool, vendor_id: Uuid) -> Result<(), String> {
        repository::delete_vendor(pool, vendor_id).await
    }

    // Page Management
    pub async fn get_all_pages(pool: &PgPool) -> Result<Vec<Page>, String> {
        repository::find_all_pages(pool).await
    }

    pub async fn get_page_by_id(pool: &PgPool, page_id: Uuid) -> Result<Page, String> {
        repository::find_page_by_id(pool, page_id).await
    }

    pub async fn create_page(pool: &PgPool, create_page: CreatePage) -> Result<Page, String> {
        repository::create_page(pool, create_page).await
    }

    pub async fn update_page(pool: &PgPool, page_id: Uuid, update_dto: AdminPageUpdateDto) -> Result<Page, String> {
        repository::update_page(pool, page_id, update_dto).await
    }

    pub async fn delete_page(pool: &PgPool, page_id: Uuid) -> Result<(), String> {
        repository::delete_page(pool, page_id).await
    }
}

