
use actix_web::{dev::ServiceRequest, Error, FromRequest, HttpRequest};
use futures::future::{Ready, ok, err};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::settings::Settings;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let config = req.app_data::<Config>().expect("Config not found");
        let auth_header = req.headers().get("Authorization");

        if let Some(header_value) = auth_header {
            if let Ok(header_str) = header_value.to_str() {
                if header_str.starts_with("Bearer ") {
                    let token = &header_str[7..];
                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
                        &Validation::default(),
                    ) {
                        Ok(token_data) => {
                            let user_id = Uuid::parse_str(&token_data.claims.sub).unwrap();
                            return ok(AuthenticatedUser { user_id });
                        }
                        Err(_) => return err(actix_web::error::ErrorUnauthorized("Invalid token")),
                    }
                }
            }
        }
        err(actix_web::error::ErrorUnauthorized("No token provided"))
    }
}

