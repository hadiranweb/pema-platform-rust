use actix_web::{web, HttpResponse, Responder};

async fn get_products() -> impl Responder {
    HttpResponse::Ok().body("This will be a list of products")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/products").route(web::get().to(get_products))
    );
}
