use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use models::{vendor::{CreateVendor, UpdateVendor}, pagination::Pagination};
use crate::{service, error::ServiceError};

pub async fn create_vendor(pool: web::Data<PgPool>, new_vendor: web::Json<CreateVendor>) -> Result<HttpResponse, ServiceError> {
    let vendor = service::create_vendor(&pool, new_vendor.into_inner()).await?;
    Ok(HttpResponse::Created().json(vendor))
}

pub async fn get_vendors(pool: web::Data<PgPool>, web::Query(pagination): web::Query<Pagination>) -> Result<HttpResponse, ServiceError> {
    let vendors = service::get_vendors(&pool, pagination).await?;
    Ok(HttpResponse::Ok().json(vendors))
}

pub async fn get_vendor_by_id(pool: web::Data<PgPool>, vendor_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    let vendor = service::get_vendor_by_id(&pool, vendor_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(vendor))
}

pub async fn update_vendor(pool: web::Data<PgPool>, vendor_id: web::Path<Uuid>, updated_vendor: web::Json<UpdateVendor>) -> Result<HttpResponse, ServiceError> {
    let vendor = service::update_vendor(&pool, vendor_id.into_inner(), updated_vendor.into_inner()).await?;
    Ok(HttpResponse::Ok().json(vendor))
}

pub async fn delete_vendor(pool: web::Data<PgPool>, vendor_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    service::delete_vendor(&pool, vendor_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

