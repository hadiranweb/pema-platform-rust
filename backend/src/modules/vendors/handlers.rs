
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::auth::middleware::AuthenticatedUser;
use crate::modules::vendors::dto::{CreateVendor, UpdateVendor};
use crate::modules::vendors::service::VendorService;

pub async fn get_all_vendors(pool: web::Data<PgPool>) -> impl Responder {
    match VendorService::get_all_vendors(pool.get_ref()).await {
        Ok(vendors) => HttpResponse::Ok().json(vendors),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_vendor_by_id(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let vendor_id = path.into_inner();
    match VendorService::get_vendor_by_id(pool.get_ref(), vendor_id).await {
        Ok(vendor) => HttpResponse::Ok().json(vendor),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

pub async fn create_vendor(pool: web::Data<PgPool>, create_vendor: web::Json<CreateVendor>, _auth_user: AuthenticatedUser) -> impl Responder {
    match VendorService::create_vendor(pool.get_ref(), create_vendor.into_inner()).await {
        Ok(vendor) => HttpResponse::Created().json(vendor),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn update_vendor(pool: web::Data<PgPool>, path: web::Path<Uuid>, update_vendor: web::Json<UpdateVendor>, _auth_user: AuthenticatedUser) -> impl Responder {
    let vendor_id = path.into_inner();
    match VendorService::update_vendor(pool.get_ref(), vendor_id, update_vendor.into_inner()).await {
        Ok(vendor) => HttpResponse::Ok().json(vendor),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn delete_vendor(pool: web::Data<PgPool>, path: web::Path<Uuid>, _auth_user: AuthenticatedUser) -> impl Responder {
    let vendor_id = path.into_inner();
    match VendorService::delete_vendor(pool.get_ref(), vendor_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

