use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use models::payment::{CreatePayment};
use crate::{service, error::ServiceError};

pub async fn process_payment(pool: web::Data<PgPool>, new_payment: web::Json<CreatePayment>) -> Result<HttpResponse, ServiceError> {
    let payment = service::process_payment(&pool, new_payment.into_inner()).await?;
    Ok(HttpResponse::Created().json(payment))
}

pub async fn get_payment_status(pool: web::Data<PgPool>, payment_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    let payment = service::get_payment_status(&pool, payment_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(payment))
}

