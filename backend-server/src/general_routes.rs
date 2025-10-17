use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::error::PemaError;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDetailsRequest {
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductInfo {
    pub id: String,
    pub name: String,
    pub price: u64,
    pub currency: String,
    pub category: String,
    pub in_stock: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInfo {
    pub id: String,
    pub status: String,
    pub total: u64,
    pub currency: String,
    pub items: Vec<OrderItem>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: String,
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: u64,
    pub total_price: u64,
}

pub async fn product_list() -> Result<HttpResponse, PemaError> {
    // پیاده‌سازی دریافت لیست محصولات از پایگاه داده
    let products: Vec<ProductInfo> = get_products_from_database().await?;
    Ok(HttpResponse::Ok().json(products))
}

async fn get_products_from_database() -> Result<Vec<ProductInfo>, PemaError> {
    // شبیه‌سازی دریافت محصولات از پایگاه داده
    // در پیاده‌سازی واقعی، این از پایگاه داده خوانده می‌شود
    let products = vec![
        ProductInfo {
            id: "1".to_string(),
            name: "لپ‌تاپ گیمینگ".to_string(),
            price: 25000000,
            currency: "IRR".to_string(),
            category: "electronics".to_string(),
            in_stock: true,
            description: Some("لپ‌تاپ قدرتمند برای بازی".to_string()),
        },
        ProductInfo {
            id: "2".to_string(),
            name: "گوشی هوشمند".to_string(),
            price: 15000000,
            currency: "IRR".to_string(),
            category: "electronics".to_string(),
            in_stock: true,
            description: Some("گوشی هوشمند با امکانات پیشرفته".to_string()),
        },
    ];
    Ok(products)
}

pub async fn order_details(path: web::Path<String>) -> Result<HttpResponse, PemaError> {
    let order_id = path.into_inner();
    // پیاده‌سازی دریافت جزئیات سفارش از پایگاه داده
    let order: OrderInfo = get_order_from_database(&order_id).await?;
    Ok(HttpResponse::Ok().json(order))
}

async fn get_order_from_database(order_id: &str) -> Result<OrderInfo, PemaError> {
    // شبیه‌سازی دریافت سفارش از پایگاه داده
    // در پیاده‌سازی واقعی، این از پایگاه داده خوانده می‌شود
    if order_id.is_empty() {
        return Err(PemaError::ValidationError("Order ID cannot be empty".to_string()));
    }

    let order = OrderInfo {
        id: order_id.to_string(),
        status: "confirmed".to_string(),
        total: 40000000,
        currency: "IRR".to_string(),
        items: vec![
            OrderItem {
                product_id: "1".to_string(),
                product_name: "لپ‌تاپ گیمینگ".to_string(),
                quantity: 1,
                unit_price: 25000000,
                total_price: 25000000,
            },
            OrderItem {
                product_id: "2".to_string(),
                product_name: "گوشی هوشمند".to_string(),
                quantity: 1,
                unit_price: 15000000,
                total_price: 15000000,
            },
        ],
        created_at: "2024-01-15T10:30:00Z".to_string(),
        updated_at: "2024-01-15T11:00:00Z".to_string(),
    };
    
    Ok(order)
}

pub fn general_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/general")
        .route("/products", web::get().to(product_list))
        .route("/orders/{order_id}", web::get().to(order_details))
    );
}

