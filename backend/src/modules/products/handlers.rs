
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::auth::middleware::AuthenticatedUser;
use crate::modules::products::dto::{CreateProduct, UpdateProduct};
use crate::modules::products::service::ProductService;
use crate::error::ServiceError;
use crate::core::plugins::manager::PluginManager;
use std::sync::Arc;

pub async fn get_all_products(pool: web::Data<PgPool>) -> impl Responder {
    match ProductService::get_all_products(pool.get_ref()).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => e.error_response(),
    }
}

pub async fn get_product_by_id(pool: web::Data<PgPool>, path: web::Path<Uuid>, plugin_manager: web::Data<Arc<PluginManager>>) -> impl Responder {
    let product_id = path.into_inner();
    match ProductService::get_product_by_id(pool.get_ref(), product_id, plugin_manager.get_ref().clone()).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => e.error_response(),
    }
}

pub async fn create_product(pool: web::Data<PgPool>, create_product: web::Json<CreateProduct>, auth_user: AuthenticatedUser) -> impl Responder {
    match ProductService::create_product(pool.get_ref(), create_product.into_inner(), auth_user.user_id).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e) => e.error_response(),
    }
}

pub async fn update_product(pool: web::Data<PgPool>, path: web::Path<Uuid>, update_product: web::Json<UpdateProduct>, auth_user: AuthenticatedUser) -> impl Responder {
    let product_id = path.into_inner();
    match ProductService::update_product(pool.get_ref(), product_id, update_product.into_inner(), auth_user.user_id).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => e.error_response(),
    }
}

pub async fn delete_product(pool: web::Data<PgPool>, path: web::Path<Uuid>, auth_user: AuthenticatedUser) -> impl Responder {
    let product_id = path.into_inner();
    match ProductService::delete_product(pool.get_ref(), product_id, auth_user.user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

