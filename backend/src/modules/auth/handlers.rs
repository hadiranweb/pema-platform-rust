
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use validator::Validate;

use crate::modules::auth::dto::{LoginRequest, RegisterRequest};
use crate::modules::auth::service::AuthService;
use crate::core::plugins::manager::PluginManager;
use std::sync::Arc;
use tokio::sync::Mutex;
use models::user::User;
use crate::error::ServiceError;

pub async fn login_user(pool: web::Data<PgPool>, form: web::Json<LoginRequest>) -> impl Responder {
    if let Err(errors) = form.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let user_login = UserLogin {
        email: form.email.clone(),
        password: form.password.clone(),
    };

    match AuthService::login_user(pool.get_ref(), user_login, config.get_ref(), form.otp_code.clone()).await {
        Ok((token, user)) => HttpResponse::Ok().json(serde_json::json!({ "token": token, "user_id": user.id, "username": user.username, "email": user.email })),
        Err(e) => e.error_response(),
    }
}

pub async fn register_user(pool: web::Data<PgPool>, form: web::Json<RegisterRequest>, config: web::Data<crate::config::settings::Settings>, plugin_manager: web::Data<Arc<PluginManager>>) -> impl Responder {
    if let Err(errors) = form.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let user_register = UserRegister {
        email: form.email.clone(),
        username: form.username.clone(),
        password: form.password.clone(),
    };

    match AuthService::register_user(pool.get_ref(), user_register, config.get_ref(), plugin_manager.get_ref().clone()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => e.error_response(),
    }
}



pub async fn generate_otp(pool: web::Data<PgPool>, auth_user: crate::modules::auth::middleware::AuthenticatedUser) -> impl Responder {
    match crate::services::otp::generate_and_store_otp(pool.get_ref(), auth_user.user_id).await {
        Ok(otp_code) => HttpResponse::Ok().json(serde_json::json!({ "message": "OTP generated successfully", "otp_code": otp_code })),
        Err(e) => ServiceError::InternalServerError(format!("Failed to generate OTP: {}", e)).error_response(),
    }
}

