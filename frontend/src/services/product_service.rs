use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;

use crate::services::review_service::Review;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub image_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub image_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub image_url: Option<String>,
}

pub struct ProductService;

impl ProductService {
    pub async fn create_product(token: &str, request: CreateProductRequest) -> Result<Product> {
        let response = Request::post("/api/products")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let product: Product = response.json().await?;
            Ok(product)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to create product: {}", error_text))
        }
    }

    pub async fn get_product_by_id(product_id: Uuid) -> Result<Product> {
        let response = Request::get(&format!("/api/products/{}", product_id))
            .send()
            .await?;

        if response.ok() {
            let product: Product = response.json().await?;
            Ok(product)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to fetch product: {}", error_text))
        }
    }

    pub async fn get_all_products() -> Result<Vec<Product>> {
        let response = Request::get("/api/products")
            .send()
            .await?;

        if response.ok() {
            let products: Vec<Product> = response.json().await?;
            Ok(products)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to fetch products: {}", error_text))
        }
    }

    pub async fn update_product(token: &str, product_id: Uuid, request: UpdateProductRequest) -> Result<Product> {
        let response = Request::put(&format!("/api/products/{}", product_id))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&request)?
            .send()
            .await?;

        if response.ok() {
            let product: Product = response.json().await?;
            Ok(product)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to update product: {}", error_text))
        }
    }

    pub async fn delete_product(token: &str, product_id: Uuid) -> Result<()> {
        let response = Request::delete(&format!("/api/products/{}", product_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?;

        if response.ok() {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to delete product: {}", error_text))
        }
    }

    pub async fn get_reviews_for_product(product_id: Uuid) -> Result<Vec<Review>> {
        let response = Request::get(&format!("/api/reviews/product/{}", product_id))
            .send()
            .await?;

        if response.ok() {
            let reviews: Vec<Review> = response.json().await?;
            Ok(reviews)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Failed to fetch product reviews: {}", error_text))
        }
    }
}

