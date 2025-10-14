use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use gloo_storage::{LocalStorage, Storage};

use crate::utils::constants::TOKEN_KEY;

// Shared Models (re-exported from backend for convenience)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category: String,
    pub vendor_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category: String,
    pub vendor_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub total_amount: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vendor {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVendor {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePage {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub is_published: bool,
}

// DTOs for Admin Updates
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminUserUpdateDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminProductUpdateDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub category: Option<String>,
    pub vendor_id: Option<Uuid>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminOrderUpdateDto {
    pub status: Option<String>,
    pub total_amount: Option<f64>,
    pub user_id: Option<Uuid>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminVendorUpdateDto {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminPageUpdateDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub slug: Option<String>,
    pub is_published: Option<bool>,
}

pub struct AdminService;

impl AdminService {
    fn get_token() -> Result<String, String> {
        LocalStorage::get(TOKEN_KEY).map_err(|_| "Token not found".to_string())
    }

    // User Management
    pub async fn get_all_users() -> Result<Vec<User>, String> {
        let token = Self::get_token()?;
        let response = Request::get("/api/admin/users")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse users: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch users: {}", error_text))
        }
    }

    pub async fn get_user_by_id(user_id: String) -> Result<User, String> {
        let token = Self::get_token()?;
        let response = Request::get(&format!("/api/admin/users/{}", user_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse user: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch user: {}", error_text))
        }
    }

    pub async fn update_user(user_id: String, update_dto: AdminUserUpdateDto) -> Result<User, String> {
        let token = Self::get_token()?;
        let response = Request::put(&format!("/api/admin/users/{}", user_id))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&update_dto)
            .map_err(|e| format!("Failed to serialize update user request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse user after update: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Update user failed: {}", error_text))
        }
    }

    pub async fn delete_user(user_id: String) -> Result<(), String> {
        let token = Self::get_token()?;
        let response = Request::delete(&format!("/api/admin/users/{}", user_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Delete user failed: {}", error_text))
        }
    }

    // Product Management
    pub async fn get_all_products() -> Result<Vec<Product>, String> {
        let token = Self::get_token()?;
        let response = Request::get("/api/admin/products")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse products: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch products: {}", error_text))
        }
    }

    pub async fn get_product_by_id(product_id: String) -> Result<Product, String> {
        let token = Self::get_token()?;
        let response = Request::get(&format!("/api/admin/products/{}", product_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse product: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch product: {}", error_text))
        }
    }

    pub async fn create_product(create_product: CreateProduct) -> Result<Product, String> {
        let token = Self::get_token()?;
        let response = Request::post("/api/admin/products")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&create_product)
            .map_err(|e| format!("Failed to serialize create product request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse product after creation: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Create product failed: {}", error_text))
        }
    }

    pub async fn update_product(product_id: String, update_dto: AdminProductUpdateDto) -> Result<Product, String> {
        let token = Self::get_token()?;
        let response = Request::put(&format!("/api/admin/products/{}", product_id))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&update_dto)
            .map_err(|e| format!("Failed to serialize update product request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse product after update: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Update product failed: {}", error_text))
        }
    }

    pub async fn delete_product(product_id: String) -> Result<(), String> {
        let token = Self::get_token()?;
        let response = Request::delete(&format!("/api/admin/products/{}", product_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Delete product failed: {}", error_text))
        }
    }

    // Order Management
    pub async fn get_all_orders() -> Result<Vec<Order>, String> {
        let token = Self::get_token()?;
        let response = Request::get("/api/admin/orders")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse orders: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch orders: {}", error_text))
        }
    }

    pub async fn get_order_by_id(order_id: String) -> Result<Order, String> {
        let token = Self::get_token()?;
        let response = Request::get(&format!("/api/admin/orders/{}", order_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse order: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch order: {}", error_text))
        }
    }

    pub async fn update_order(order_id: String, update_dto: AdminOrderUpdateDto) -> Result<Order, String> {
        let token = Self::get_token()?;
        let response = Request::put(&format!("/api/admin/orders/{}", order_id))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&update_dto)
            .map_err(|e| format!("Failed to serialize update order request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse order after update: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Update order failed: {}", error_text))
        }
    }

    pub async fn delete_order(order_id: String) -> Result<(), String> {
        let token = Self::get_token()?;
        let response = Request::delete(&format!("/api/admin/orders/{}", order_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Delete order failed: {}", error_text))
        }
    }

    // Vendor Management
    pub async fn get_all_vendors() -> Result<Vec<Vendor>, String> {
        let token = Self::get_token()?;
        let response = Request::get("/api/admin/vendors")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse vendors: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch vendors: {}", error_text))
        }
    }

    pub async fn get_vendor_by_id(vendor_id: String) -> Result<Vendor, String> {
        let token = Self::get_token()?;
        let response = Request::get(&format!("/api/admin/vendors/{}", vendor_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse vendor: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch vendor: {}", error_text))
        }
    }

    pub async fn create_vendor(create_vendor: CreateVendor) -> Result<Vendor, String> {
        let token = Self::get_token()?;
        let response = Request::post("/api/admin/vendors")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&create_vendor)
            .map_err(|e| format!("Failed to serialize create vendor request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse vendor after creation: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Create vendor failed: {}", error_text))
        }
    }

    pub async fn update_vendor(vendor_id: String, update_dto: AdminVendorUpdateDto) -> Result<Vendor, String> {
        let token = Self::get_token()?;
        let response = Request::put(&format!("/api/admin/vendors/{}", vendor_id))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&update_dto)
            .map_err(|e| format!("Failed to serialize update vendor request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse vendor after update: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Update vendor failed: {}", error_text))
        }
    }

    pub async fn delete_vendor(vendor_id: String) -> Result<(), String> {
        let token = Self::get_token()?;
        let response = Request::delete(&format!("/api/admin/vendors/{}", vendor_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Delete vendor failed: {}", error_text))
        }
    }

    // Page Management
    pub async fn get_all_pages() -> Result<Vec<Page>, String> {
        let token = Self::get_token()?;
        let response = Request::get("/api/admin/pages")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse pages: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch pages: {}", error_text))
        }
    }

    pub async fn get_page_by_id(page_id: String) -> Result<Page, String> {
        let token = Self::get_token()?;
        let response = Request::get(&format!("/api/admin/pages/{}", page_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse page: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Failed to fetch page: {}", error_text))
        }
    }

    pub async fn create_page(create_page: CreatePage) -> Result<Page, String> {
        let token = Self::get_token()?;
        let response = Request::post("/api/admin/pages")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&create_page)
            .map_err(|e| format!("Failed to serialize create page request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse page after creation: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Create page failed: {}", error_text))
        }
    }

    pub async fn update_page(page_id: String, update_dto: AdminPageUpdateDto) -> Result<Page, String> {
        let token = Self::get_token()?;
        let response = Request::put(&format!("/api/admin/pages/{}", page_id))
            .header("Authorization", &format!("Bearer {}", token))
            .json(&update_dto)
            .map_err(|e| format!("Failed to serialize update page request: {}", e.to_string()))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            response.json().await.map_err(|e| format!("Failed to parse page after update: {}", e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Update page failed: {}", error_text))
        }
    }

    pub async fn delete_page(page_id: String) -> Result<(), String> {
        let token = Self::get_token()?;
        let response = Request::delete(&format!("/api/admin/pages/{}", page_id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e.to_string()))?;

        if response.ok() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("Delete page failed: {}", error_text))
        }
    }
}

