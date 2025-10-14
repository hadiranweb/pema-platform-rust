
use actix_web::web;

use crate::modules::auth::handlers::{login_user, register_user};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user)),
    );
}

