use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
// use wasm_general_backend::{get_product_list, get_order_details};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDetailsRequest {
    pub order_id: String,
}

pub async fn product_list() -> impl Responder {
    // TODO: Implement get_product_list function
    match Ok::<Vec<String>, String>(vec![]) { // get_product_list().await {
        Ok(products_js_value) => {
            let products_str = serde_json::to_string(&products_js_value).unwrap_or_default();
            HttpResponse::Ok().body(products_str)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to get product list: {:?}", e)),
    }
}

pub async fn order_details(path: web::Path<String>) -> impl Responder {
    let order_id = path.into_inner();
    // TODO: Implement get_order_details function
    match Ok::<serde_json::Value, String>(serde_json::json!({})) { // get_order_details(order_id.clone()).await {
        Ok(details_js_value) => {
            let details_str = serde_json::to_string(&details_js_value).unwrap_or_default();
            HttpResponse::Ok().body(details_str)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to get order details: {:?}", e)),
    }
}

pub fn general_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/general")
        .route("/products", web::get().to(product_list))
        .route("/orders/{order_id}", web::get().to(order_details))
    );
}

