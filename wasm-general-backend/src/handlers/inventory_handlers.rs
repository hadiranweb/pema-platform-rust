use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use models::{inventory::{CreateInventoryItem, UpdateInventoryItem}, pagination::Pagination};
use crate::{service, error::ServiceError};

pub async fn create_inventory_item(pool: web::Data<PgPool>, new_item: web::Json<CreateInventoryItem>) -> Result<HttpResponse, ServiceError> {
    let item = service::create_inventory_item(&pool, new_item.into_inner()).await?;
    Ok(HttpResponse::Created().json(item))
}

pub async fn get_inventory_items(pool: web::Data<PgPool>, web::Query(pagination): web::Query<Pagination>) -> Result<HttpResponse, ServiceError> {
    let items = service::get_inventory_items(&pool, pagination).await?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn get_inventory_item_by_id(pool: web::Data<PgPool>, item_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    let item = service::get_inventory_item_by_id(&pool, item_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(item))
}

pub async fn update_inventory_item(pool: web::Data<PgPool>, item_id: web::Path<Uuid>, updated_item: web::Json<UpdateInventoryItem>) -> Result<HttpResponse, ServiceError> {
    let item = service::update_inventory_item(&pool, item_id.into_inner(), updated_item.into_inner()).await?;
    Ok(HttpResponse::Ok().json(item))
}

pub async fn delete_inventory_item(pool: web::Data<PgPool>, item_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    service::delete_inventory_item(&pool, item_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

