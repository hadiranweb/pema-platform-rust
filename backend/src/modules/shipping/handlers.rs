use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use crate::error::ServiceError;
use super::{service, dto};

pub async fn create_shipping(pool: web::Data<PgPool>, new_shipping: web::Json<dto::CreateShippingDto>) -> Result<HttpResponse, ServiceError> {
    let shipping = service::create_shipping(
        &pool,
        new_shipping.order_id,
        new_shipping.address.clone(),
        new_shipping.city.clone(),
        new_shipping.state.clone(),
        new_shipping.zip_code.clone(),
        new_shipping.country.clone(),
    ).await?;
    Ok(HttpResponse::Created().json(shipping))
}

pub async fn get_shipping(pool: web::Data<PgPool>, shipping_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    let shipping = service::get_shipping_details(&pool, shipping_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(shipping))
}

pub async fn update_shipping_status(pool: web::Data<PgPool>, shipping_id: web::Path<Uuid>, status_update: web::Json<dto::UpdateShippingStatusDto>) -> Result<HttpResponse, ServiceError> {
    let shipping = service::update_shipping_status(&pool, shipping_id.into_inner(), &status_update.status).await?;
    Ok(HttpResponse::Ok().json(shipping))
}

