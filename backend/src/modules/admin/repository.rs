
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use models::user::User;
use models::product::{Product, CreateProduct};
use models::order::{Order, CreateOrder};
use models::vendor::{Vendor, CreateVendor};
use models::page::{Page, CreatePage};

use crate::modules::admin::dto::{AdminUserUpdateDto, AdminProductUpdateDto, AdminOrderUpdateDto, AdminVendorUpdateDto, AdminPageUpdateDto};

// User Management
pub async fn find_all_users(pool: &PgPool) -> Result<Vec<User>, String> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch all users: {}", e))
}

pub async fn find_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User, String> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch user by ID: {}", e))?
        .ok_or_else(|| "User not found".to_string())
}

pub async fn update_user(pool: &PgPool, user_id: Uuid, update_dto: AdminUserUpdateDto) -> Result<User, String> {
    sqlx::query_as::<_, User>(
        "UPDATE users SET username = COALESCE($1, username), email = COALESCE($2, email), role = COALESCE($3, role), updated_at = NOW() WHERE id = $4 RETURNING *"
    )
    .bind(update_dto.username)
    .bind(update_dto.email)
    .bind(update_dto.role)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to update user: {}", e))
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete user: {}", e))?;
    Ok(())
}

// Product Management
pub async fn find_all_products(pool: &PgPool) -> Result<Vec<Product>, String> {
    sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch all products: {}", e))
}

pub async fn find_product_by_id(pool: &PgPool, product_id: Uuid) -> Result<Product, String> {
    sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
        .bind(product_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch product by ID: {}", e))?
        .ok_or_else(|| "Product not found".to_string())
}

pub async fn create_product(pool: &PgPool, create_product: CreateProduct) -> Result<Product, String> {
    sqlx::query_as::<_, Product>(
        "INSERT INTO products (id, name, description, price, stock, category, vendor_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(create_product.name)
    .bind(create_product.description)
    .bind(create_product.price)
    .bind(create_product.stock)
    .bind("General") // Default category for now
    .bind(Uuid::new_v4()) // Placeholder vendor_id
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to create product: {}", e))
}

pub async fn update_product(pool: &PgPool, product_id: Uuid, update_dto: AdminProductUpdateDto) -> Result<Product, String> {
    sqlx::query_as::<_, Product>(
        "UPDATE products SET name = COALESCE($1, name), description = COALESCE($2, description), price = COALESCE($3, price), stock = COALESCE($4, stock), category = COALESCE($5, category), vendor_id = COALESCE($6, vendor_id), updated_at = NOW() WHERE id = $7 RETURNING *"
    )
    .bind(update_dto.name)
    .bind(update_dto.description)
    .bind(update_dto.price)
    .bind(update_dto.stock)
    .bind(update_dto.category)
    .bind(update_dto.vendor_id)
    .bind(product_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to update product: {}", e))
}

pub async fn delete_product(pool: &PgPool, product_id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(product_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete product: {}", e))?;
    Ok(())
}

// Order Management
pub async fn find_all_orders(pool: &PgPool) -> Result<Vec<Order>, String> {
    sqlx::query_as::<_, Order>("SELECT * FROM orders")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch all orders: {}", e))
}

pub async fn find_order_by_id(pool: &PgPool, order_id: Uuid) -> Result<Order, String> {
    sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(order_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch order by ID: {}", e))?
        .ok_or_else(|| "Order not found".to_string())
}

pub async fn update_order(pool: &PgPool, order_id: Uuid, update_dto: AdminOrderUpdateDto) -> Result<Order, String> {
    sqlx::query_as::<_, Order>(
        "UPDATE orders SET status = COALESCE($1, status), total_amount = COALESCE($2, total_amount), user_id = COALESCE($3, user_id), updated_at = NOW() WHERE id = $4 RETURNING *"
    )
    .bind(update_dto.status)
    .bind(update_dto.total_amount)
    .bind(update_dto.user_id)
    .bind(order_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to update order: {}", e))
}

pub async fn delete_order(pool: &PgPool, order_id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM orders WHERE id = $1")
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete order: {}", e))?;
    Ok(())
}

// Vendor Management
pub async fn find_all_vendors(pool: &PgPool) -> Result<Vec<Vendor>, String> {
    sqlx::query_as::<_, Vendor>("SELECT * FROM vendors")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch all vendors: {}", e))
}

pub async fn find_vendor_by_id(pool: &PgPool, vendor_id: Uuid) -> Result<Vendor, String> {
    sqlx::query_as::<_, Vendor>("SELECT * FROM vendors WHERE id = $1")
        .bind(vendor_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch vendor by ID: {}", e))?
        .ok_or_else(|| "Vendor not found".to_string())
}

pub async fn create_vendor(pool: &PgPool, create_vendor: CreateVendor) -> Result<Vendor, String> {
    sqlx::query_as::<_, Vendor>(
        "INSERT INTO vendors (id, name, description) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(create_vendor.name)
    .bind(create_vendor.description)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to create vendor: {}", e))
}

pub async fn update_vendor(pool: &PgPool, vendor_id: Uuid, update_dto: AdminVendorUpdateDto) -> Result<Vendor, String> {
    sqlx::query_as::<_, Vendor>(
        "UPDATE vendors SET name = COALESCE($1, name), description = COALESCE($2, description), updated_at = NOW() WHERE id = $3 RETURNING *"
    )
    .bind(update_dto.name)
    .bind(update_dto.description)
    .bind(vendor_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to update vendor: {}", e))
}

pub async fn delete_vendor(pool: &PgPool, vendor_id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM vendors WHERE id = $1")
        .bind(vendor_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete vendor: {}", e))?;
    Ok(())
}

// Page Management
pub async fn find_all_pages(pool: &PgPool) -> Result<Vec<Page>, String> {
    sqlx::query_as::<_, Page>("SELECT * FROM pages")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch all pages: {}", e))
}

pub async fn find_page_by_id(pool: &PgPool, page_id: Uuid) -> Result<Page, String> {
    sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE id = $1")
        .bind(page_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch page by ID: {}", e))?
        .ok_or_else(|| "Page not found".to_string())
}

pub async fn create_page(pool: &PgPool, create_page: CreatePage) -> Result<Page, String> {
    sqlx::query_as::<_, Page>(
        "INSERT INTO pages (id, title, content, slug, is_published) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(create_page.title)
    .bind(create_page.content)
    .bind(create_page.slug)
    .bind(create_page.is_published)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to create page: {}", e))
}

pub async fn update_page(pool: &PgPool, page_id: Uuid, update_dto: AdminPageUpdateDto) -> Result<Page, String> {
    sqlx::query_as::<_, Page>(
        "UPDATE pages SET title = COALESCE($1, title), content = COALESCE($2, content), slug = COALESCE($3, slug), is_published = COALESCE($4, is_published), updated_at = NOW() WHERE id = $5 RETURNING *"
    )
    .bind(update_dto.title)
    .bind(update_dto.content)
    .bind(update_dto.slug)
    .bind(update_dto.is_published)
    .bind(page_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to update page: {}", e))
}

pub async fn delete_page(pool: &PgPool, page_id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM pages WHERE id = $1")
        .bind(page_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete page: {}", e))?;
    Ok(())
}

