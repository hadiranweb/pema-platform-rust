use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use models::{user::{CreateUser, UpdateUser}, pagination::Pagination};
use crate::{service, error::ServiceError};

pub async fn create_user(pool: web::Data<PgPool>, new_user: web::Json<CreateUser>) -> Result<HttpResponse, ServiceError> {
    let user = service::create_user(&pool, new_user.into_inner()).await?;
    Ok(HttpResponse::Created().json(user))
}

pub async fn get_users(pool: web::Data<PgPool>, web::Query(pagination): web::Query<Pagination>) -> Result<HttpResponse, ServiceError> {
    let users = service::get_users(&pool, pagination).await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user_by_id(pool: web::Data<PgPool>, user_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    let user = service::get_user_by_id(&pool, user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_user(pool: web::Data<PgPool>, user_id: web::Path<Uuid>, updated_user: web::Json<UpdateUser>) -> Result<HttpResponse, ServiceError> {
    let user = service::update_user(&pool, user_id.into_inner(), updated_user.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete_user(pool: web::Data<PgPool>, user_id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    service::delete_user(&pool, user_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

