use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use models::{order::{CreateOrder, UpdateOrder}, pagination::Pagination};
use crate::{service, error::ServiceError};

pub async fn create_order(pool: web::Data<PgPool>, new_order: web::Json<CreateOrder>) -> Result<HttpResponse, ServiceError> {
    let order = service::create_order(&pool, new_order.into_inner()).await?;
    Ok(HttpResponse::Created().json(order))
}

pub async fn get_orders(pool: web::Data<PgPool>, web::Query(pagination): web::Query<Pagination>) -> Result<HttpResponse, ServiceError> {
    let orders = service::get_orders(&pool, pagination).await?;
    Ok(HttpResponse::Ok().json(orders))
}

pub async fn get_order_by_id(pool: web::Data<PgPool>, order_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    let order = service::get_order_by_id(&pool, order_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(order))
}

pub async fn update_order(pool: web::Data<PgPool>, order_id: web::Path<Uuid>, updated_order: web::Json<UpdateOrder>) -> Result<HttpResponse, ServiceError> {
    let order = service::update_order(&pool, order_id.into_inner(), updated_order.into_inner()).await?;
    Ok(HttpResponse::Ok().json(order))
}

pub async fn delete_order(pool: web::Data<PgPool>, order_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    service::delete_order(&pool, order_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

