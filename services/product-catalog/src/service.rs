use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use models::{product::{Product, CreateProduct, UpdateProduct}, pagination::{Pagination, PaginatedResponse}};
use crate::error::ServiceError;

pub async fn create_product(pool: &PgPool, new_product: CreateProduct) -> Result<Product, ServiceError> {
    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products (name, description, price, stock, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(new_product.name)
    .bind(new_product.description)
    .bind(new_product.price)
    .bind(new_product.stock)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(product)
}

pub async fn get_products(pool: &PgPool, pagination: Pagination) -> Result<PaginatedResponse<Product>, ServiceError> {
    let offset = (pagination.page - 1) * pagination.limit;

    let total_items: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
        .fetch_one(pool)
        .await?;

    let products = sqlx::query_as::<_, Product>(
        "SELECT * FROM products ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(pagination.limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await?;

    let total_pages = (total_items as f64 / pagination.limit as f64).ceil() as u32;

    Ok(PaginatedResponse {
        items: products,
        total_items: total_items as u32,
        current_page: pagination.page,
        total_pages,
        limit: pagination.limit,
    })
}

pub async fn get_product_by_id(pool: &PgPool, product_id: Uuid) -> Result<Product, ServiceError> {
    let product = sqlx::query_as::<_, Product>(
        "SELECT * FROM products WHERE id = $1"
    )
    .bind(product_id)
    .fetch_one(pool)
    .await?;

    Ok(product)
}

pub async fn update_product(pool: &PgPool, product_id: Uuid, updated_product: UpdateProduct) -> Result<Product, ServiceError> {
    let existing_product = get_product_by_id(pool, product_id).await?;

    let product = sqlx::query_as::<_, Product>(
        "UPDATE products SET name = $1, description = $2, price = $3, stock = $4, updated_at = $5 WHERE id = $6 RETURNING *"
    )
    .bind(updated_product.name.unwrap_or(existing_product.name))
    .bind(updated_product.description.or_else(|| existing_product.description))
    .bind(updated_product.price.unwrap_or(existing_product.price))
    .bind(updated_product.stock.unwrap_or(existing_product.stock))
    .bind(Utc::now())
    .bind(product_id)
    .fetch_one(pool)
    .await?;

    Ok(product)
}

pub async fn delete_product(pool: &PgPool, product_id: Uuid) -> Result<(), ServiceError> {
    sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(product_id)
        .execute(pool)
        .await?;

    Ok(())
}

