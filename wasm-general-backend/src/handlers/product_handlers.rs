use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use models::{product::{CreateProduct, UpdateProduct}, pagination::Pagination};
use crate::{service, error::ServiceError};

pub async fn create_product(pool: web::Data<PgPool>, new_product: web::Json<CreateProduct>) -> Result<HttpResponse, ServiceError> {
    let product = service::create_product(&pool, new_product.into_inner()).await?;
    Ok(HttpResponse::Created().json(product))
}

pub async fn get_products(pool: web::Data<PgPool>, web::Query(pagination): web::Query<Pagination>) -> Result<HttpResponse, ServiceError> {
    let products = service::get_products(&pool, pagination).await?;
    Ok(HttpResponse::Ok().json(products))
}

pub async fn get_product_by_id(pool: web::Data<PgPool>, product_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    let product = service::get_product_by_id(&pool, product_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(product))
}

pub async fn update_product(pool: web::Data<PgPool>, product_id: web::Path<Uuid>, updated_product: web::Json<UpdateProduct>) -> Result<HttpResponse, ServiceError> {
    let product = service::update_product(&pool, product_id.into_inner(), updated_product.into_inner()).await?;
    Ok(HttpResponse::Ok().json(product))
}

pub async fn delete_product(pool: web::Data<PgPool>, product_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    service::delete_product(&pool, product_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

