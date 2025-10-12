
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use validator::Validate;

use crate::modules::auth::dto::{LoginRequest, RegisterRequest};
use crate::modules::auth::service::AuthService;
use crate::shared::config::config::Config;
use crate::core::plugins::manager::PluginManager;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::shared::models::user::{UserLogin, UserRegister};
use crate::error::ServiceError;

pub async fn login_user(pool: web::Data<PgPool>, form: web::Json<LoginRequest>, config: web::Data<Config>) -> impl Responder {
    if let Err(errors) = form.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let user_login = UserLogin {
        email: form.email.clone(),
        password: form.password.clone(),
    };

    match AuthService::login_user(pool.get_ref(), user_login, config.get_ref()).await {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
        Err(e) => e.error_response(),
    }
}

pub async fn register_user(pool: web::Data<PgPool>, form: web::Json<RegisterRequest>, config: web::Data<Config>, plugin_manager: web::Data<Arc<PluginManager>>) -> impl Responder {
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

