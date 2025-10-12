use actix_web::{web, HttpResponse, Responder};

async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login endpoint")
}

async fn register() -> impl Responder {
    HttpResponse::Ok().body("Register endpoint")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/auth/login").route(web::post().to(login)))
       .service(web::resource("/auth/register").route(web::post().to(register)));
}

