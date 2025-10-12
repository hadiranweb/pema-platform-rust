
use actix_web::web;

use crate::modules::admin::handlers::{create_page, create_product, create_vendor, delete_order, delete_page, delete_product, delete_user, delete_vendor, get_all_orders, get_all_pages, get_all_products, get_all_vendors, get_all_users, get_order_by_id, get_page_by_id, get_product_by_id, get_user_by_id, get_vendor_by_id, update_order, update_page, update_product, update_user, update_vendor};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            // User Management
            .route("/users", web::get().to(get_all_users))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
            // Product Management
            .route("/products", web::get().to(get_all_products))
            .route("/products", web::post().to(create_product))
            .route("/products/{id}", web::get().to(get_product_by_id))
            .route("/products/{id}", web::put().to(update_product))
            .route("/products/{id}", web::delete().to(delete_product))
            // Order Management
            .route("/orders", web::get().to(get_all_orders))
            .route("/orders/{id}", web::get().to(get_order_by_id))
            .route("/orders/{id}", web::put().to(update_order))
            .route("/orders/{id}", web::delete().to(delete_order))
            // Vendor Management
            .route("/vendors", web::get().to(get_all_vendors))
            .route("/vendors", web::post().to(create_vendor))
            .route("/vendors/{id}", web::get().to(get_vendor_by_id))
            .route("/vendors/{id}", web::put().to(update_vendor))
            .route("/vendors/{id}", web::delete().to(delete_vendor))
            // Page Management
            .route("/pages", web::get().to(get_all_pages))
            .route("/pages", web::post().to(create_page))
            .route("/pages/{id}", web::get().to(get_page_by_id))
            .route("/pages/{id}", web::put().to(update_page))
            .route("/pages/{id}", web::delete().to(delete_page)),
    );
}

