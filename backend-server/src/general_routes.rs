use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
// TODO: Implement proper product and order management
// use wasm_general_backend::{get_product_list, get_order_details};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDetailsRequest {
    pub order_id: String,
}

pub async fn product_list() -> impl Responder {
    // TODO: Implement proper product list retrieval
    let mock_products = serde_json::json!([
        {"id": "1", "name": "Sample Product 1", "price": 100},
        {"id": "2", "name": "Sample Product 2", "price": 200}
    ]);
    HttpResponse::Ok().json(mock_products)
}

pub async fn order_details(path: web::Path<String>) -> impl Responder {
    let order_id = path.into_inner();
    // TODO: Implement proper order details retrieval
    let mock_order = serde_json::json!({
        "id": order_id,
        "status": "pending",
        "total": 300,
        "items": [
            {"product_id": "1", "quantity": 2, "price": 100},
            {"product_id": "2", "quantity": 1, "price": 200}
        ]
    });
    HttpResponse::Ok().json(mock_order)
}

pub fn general_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/general")
        .route("/products", web::get().to(product_list))
        .route("/orders/{order_id}", web::get().to(order_details))
    );
}

