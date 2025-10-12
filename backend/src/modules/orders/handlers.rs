
use actix_web::{web, HttpResponse, Responder};
use actix_web::ResponseError;
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::auth::middleware::AuthenticatedUser;
use crate::modules::orders::dto::{CreateOrder, UpdateOrder};
use crate::modules::orders::service::OrderService;
use crate::core::plugins::manager::PluginManager;
use std::sync::Arc;

pub async fn get_all_orders(pool: web::Data<PgPool>) -> impl Responder {
    match OrderService::get_all_orders(pool.get_ref()).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => e.error_response(),
    }
}

pub async fn get_order_by_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let order_id = path.into_inner();
    match OrderService::get_order_by_id(pool.get_ref(), order_id).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => e.error_response(),
    }
}

pub async fn create_order(pool: web::Data<PgPool>, create_order: web::Json<CreateOrder>, auth_user: AuthenticatedUser, plugin_manager: web::Data<Arc<PluginManager>>) -> impl Responder {
    match OrderService::create_order(pool.get_ref(), create_order.into_inner(), auth_user.user_id, plugin_manager.get_ref().clone()).await {
        Ok(order) => HttpResponse::Created().json(order),
        Err(e) => e.error_response(),
    }
}

pub async fn update_order(pool: web::Data<PgPool>, path: web::Path<Uuid>, update_order: web::Json<UpdateOrder>, auth_user: AuthenticatedUser) -> impl Responder {
    let order_id = path.into_inner();
    match OrderService::update_order(pool.get_ref(), order_id, update_order.into_inner(), auth_user.user_id).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => e.error_response(),
    }
}

pub async fn delete_order(pool: web::Data<PgPool>, path: web::Path<Uuid>, auth_user: AuthenticatedUser) -> impl Responder {
    let order_id = path.into_inner();
    match OrderService::delete_order(pool.get_ref(), order_id, auth_user.user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn get_user_orders(pool: web::Data<PgPool>, auth_user: AuthenticatedUser) -> impl Responder {
    match OrderService::get_user_orders(pool.get_ref(), auth_user.user_id).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => e.error_response(),
    }
}

