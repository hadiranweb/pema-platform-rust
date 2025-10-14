use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::error::ServiceError;
use super::{service, dto};

pub async fn create_page(pool: web::Data<PgPool>, new_page: web::Json<dto::CreatePageDto>) -> Result<HttpResponse, ServiceError> {
    let page = service::create_page(
        &pool,
        new_page.title.clone(),
        new_page.slug.clone(),
        new_page.content.clone(),
        new_page.is_published,
    ).await?;
    Ok(HttpResponse::Created().json(page))
}

pub async fn get_page(pool: web::Data<PgPool>, page_id: web::Path<i32>) -> Result<HttpResponse, ServiceError> {
    let page = service::get_page_details(&pool, page_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(page))
}

pub async fn update_page(pool: web::Data<PgPool>, page_id: web::Path<i32>, updated_page: web::Json<dto::UpdatePageDto>) -> Result<HttpResponse, ServiceError> {
    let page = service::update_page(
        &pool,
        page_id.into_inner(),
        updated_page.title.clone(),
        updated_page.slug.clone(),
        updated_page.content.clone(),
        updated_page.is_published,
    ).await?;
    Ok(HttpResponse::Ok().json(page))
}

pub async fn delete_page(pool: web::Data<PgPool>, page_id: web::Path<i32>) -> Result<HttpResponse, ServiceError> {
    let page = service::delete_page(&pool, page_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(page))
}

pub async fn get_all_pages(pool: web::Data<PgPool>) -> Result<HttpResponse, ServiceError> {
    let pages = service::get_all_pages(&pool).await?;
    Ok(HttpResponse::Ok().json(pages))
}

pub async fn get_published_pages(pool: web::Data<PgPool>) -> Result<HttpResponse, ServiceError> {
    let pages = service::get_published_pages(&pool).await?;
    Ok(HttpResponse::Ok().json(pages))
}

