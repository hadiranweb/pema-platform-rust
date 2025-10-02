use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use models::{notification::{CreateNotification, UpdateNotification}, pagination::Pagination};
use crate::{service, error::ServiceError};

pub async fn create_notification(pool: web::Data<PgPool>, new_notification: web::Json<CreateNotification>) -> Result<HttpResponse, ServiceError> {
    let notification = service::create_notification(&pool, new_notification.into_inner()).await?;
    Ok(HttpResponse::Created().json(notification))
}

pub async fn get_notifications(pool: web::Data<PgPool>, web::Query(pagination): web::Query<Pagination>) -> Result<HttpResponse, ServiceError> {
    let notifications = service::get_notifications(&pool, pagination).await?;
    Ok(HttpResponse::Ok().json(notifications))
}

pub async fn get_notification_by_id(pool: web::Data<PgPool>, notification_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    let notification = service::get_notification_by_id(&pool, notification_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(notification))
}

pub async fn update_notification(pool: web::Data<PgPool>, notification_id: web::Path<Uuid>, updated_notification: web::Json<UpdateNotification>) -> Result<HttpResponse, ServiceError> {
    let notification = service::update_notification(&pool, notification_id.into_inner(), updated_notification.into_inner()).await?;
    Ok(HttpResponse::Ok().json(notification))
}

pub async fn delete_notification(pool: web::Data<PgPool>, notification_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    service::delete_notification(&pool, notification_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

