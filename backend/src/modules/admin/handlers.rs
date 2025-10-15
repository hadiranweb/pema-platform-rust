
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::admin::dto::{AdminUserUpdateDto, AdminProductUpdateDto, AdminOrderUpdateDto, AdminVendorUpdateDto, AdminPageUpdateDto};
use crate::modules::admin::service::AdminService;
use models::product::CreateProduct;
use models::vendor::CreateVendor;
use models::page::CreatePage;

// User Management Handlers
pub async fn get_all_users(pool: web::Data<PgPool>) -> impl Responder {
    match AdminService::get_all_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_user_by_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let user_id = path.into_inner();
    match AdminService::get_user_by_id(pool.get_ref(), user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

pub async fn update_user(pool: web::Data<PgPool>, path: web::Path<Uuid>, update_dto: web::Json<AdminUserUpdateDto>) -> impl Responder {
    let user_id = path.into_inner();
    match AdminService::update_user(pool.get_ref(), user_id, update_dto.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn delete_user(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let user_id = path.into_inner();
    match AdminService::delete_user(pool.get_ref(), user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// Product Management Handlers
pub async fn get_all_products(pool: web::Data<PgPool>) -> impl Responder {
    match AdminService::get_all_products(pool.get_ref()).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_product_by_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let product_id = path.into_inner();
    match AdminService::get_product_by_id(pool.get_ref(), product_id).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

pub async fn create_product(pool: web::Data<PgPool>, create_product: web::Json<CreateProduct>) -> impl Responder {
    match AdminService::create_product(pool.get_ref(), create_product.into_inner()).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn update_product(pool: web::Data<PgPool>, path: web::Path<Uuid>, update_dto: web::Json<AdminProductUpdateDto>) -> impl Responder {
    let product_id = path.into_inner();
    match AdminService::update_product(pool.get_ref(), product_id, update_dto.into_inner()).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn delete_product(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let product_id = path.into_inner();
    match AdminService::delete_product(pool.get_ref(), product_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// Order Management Handlers
pub async fn get_all_orders(pool: web::Data<PgPool>) -> impl Responder {
    match AdminService::get_all_orders(pool.get_ref()).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_order_by_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let order_id = path.into_inner();
    match AdminService::get_order_by_id(pool.get_ref(), order_id).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

pub async fn update_order(pool: web::Data<PgPool>, path: web::Path<Uuid>, update_dto: web::Json<AdminOrderUpdateDto>) -> impl Responder {
    let order_id = path.into_inner();
    match AdminService::update_order(pool.get_ref(), order_id, update_dto.into_inner()).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn delete_order(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let order_id = path.into_inner();
    match AdminService::delete_order(pool.get_ref(), order_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// Vendor Management Handlers
pub async fn get_all_vendors(pool: web::Data<PgPool>) -> impl Responder {
    match AdminService::get_all_vendors(pool.get_ref()).await {
        Ok(vendors) => HttpResponse::Ok().json(vendors),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_vendor_by_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let vendor_id = path.into_inner();
    match AdminService::get_vendor_by_id(pool.get_ref(), vendor_id).await {
        Ok(vendor) => HttpResponse::Ok().json(vendor),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

pub async fn create_vendor(pool: web::Data<PgPool>, create_vendor: web::Json<CreateVendor>) -> impl Responder {
    match AdminService::create_vendor(pool.get_ref(), create_vendor.into_inner()).await {
        Ok(vendor) => HttpResponse::Created().json(vendor),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn update_vendor(pool: web::Data<PgPool>, path: web::Path<Uuid>, update_dto: web::Json<AdminVendorUpdateDto>) -> impl Responder {
    let vendor_id = path.into_inner();
    match AdminService::update_vendor(pool.get_ref(), vendor_id, update_dto.into_inner()).await {
        Ok(vendor) => HttpResponse::Ok().json(vendor),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn delete_vendor(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let vendor_id = path.into_inner();
    match AdminService::delete_vendor(pool.get_ref(), vendor_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// Page Management Handlers
pub async fn get_all_pages(pool: web::Data<PgPool>) -> impl Responder {
    match AdminService::get_all_pages(pool.get_ref()).await {
        Ok(pages) => HttpResponse::Ok().json(pages),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_page_by_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let page_id = path.into_inner();
    match AdminService::get_page_by_id(pool.get_ref(), page_id).await {
        Ok(page) => HttpResponse::Ok().json(page),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

pub async fn create_page(pool: web::Data<PgPool>, create_page: web::Json<CreatePage>) -> impl Responder {
    match AdminService::create_page(pool.get_ref(), create_page.into_inner()).await {
        Ok(page) => HttpResponse::Created().json(page),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn update_page(pool: web::Data<PgPool>, path: web::Path<Uuid>, update_dto: web::Json<AdminPageUpdateDto>) -> impl Responder {
    let page_id = path.into_inner();
    match AdminService::update_page(pool.get_ref(), page_id, update_dto.into_inner()).await {
        Ok(page) => HttpResponse::Ok().json(page),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn delete_page(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let page_id = path.into_inner();
    match AdminService::delete_page(pool.get_ref(), page_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

