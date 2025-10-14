
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::shared::models::product::{CreateProduct, Product, UpdateProduct};

pub async fn create_product(pool: &PgPool, create_product: CreateProduct, vendor_id: Uuid) -> Result<Product, String> {
    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products (id, name, description, price, stock, category, vendor_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(create_product.name)
    .bind(create_product.description)
    .bind(create_product.price)
    .bind(create_product.stock)
    .bind("General") // Default category for now
    .bind(vendor_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to create product: {}", e))?;

    Ok(product)
}

pub async fn find_product_by_id(pool: &PgPool, product_id: Uuid) -> Result<Product, String> {
    let product = sqlx::query_as::<_, Product>(
        "SELECT * FROM products WHERE id = $1"
    )
    .bind(product_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to fetch product by ID: {}", e))?
    .ok_or_else(|| "Product not found".to_string())?;

    Ok(product)
}

pub async fn find_all_products(pool: &PgPool) -> Result<Vec<Product>, String> {
    let products = sqlx::query_as::<_, Product>(
        "SELECT * FROM products"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch all products: {}", e))?;

    Ok(products)
}

pub async fn update_product(pool: &PgPool, product_id: Uuid, update_product: UpdateProduct) -> Result<Product, String> {
    let product = sqlx::query_as::<_, Product>(
        "UPDATE products SET name = COALESCE($1, name), description = COALESCE($2, description), price = COALESCE($3, price), stock = COALESCE($4, stock), updated_at = NOW() WHERE id = $5 RETURNING *"
    )
    .bind(update_product.name)
    .bind(update_product.description)
    .bind(update_product.price)
    .bind(update_product.stock)
    .bind(product_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to update product: {}", e))?;

    Ok(product)
}

pub async fn delete_product(pool: &PgPool, product_id: Uuid) -> Result<(), String> {
    sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(product_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete product: {}", e))?;

    Ok(())
}

