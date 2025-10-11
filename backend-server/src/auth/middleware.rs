use actix_web::{dev::{ServiceRequest}, Error, HttpMessage, web, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::{ready, Ready};
use actix_web::FromRequest;

use crate::auth::utils::{validate_jwt, Claims};
use shared_config::config::AppConfig;
use crate::wallet::errors::WalletError;

pub struct AuthenticatedUser {
    pub claims: Claims,
}

impl FromRequest for AuthenticatedUser {
    type Error = WalletError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(claims) = req.extensions().get::<Claims>() {
            ready(Ok(AuthenticatedUser { claims: claims.clone() }))
        } else {
            ready(Err(WalletError::UnauthorizedAdminAction("Authentication claims not found in request extensions".to_string())))
        }
    }
}

pub async fn validate_jwt_from_bearer(req: ServiceRequest, bearer: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req.app_data::<web::Data<AppConfig>>().expect("AppConfig not found").clone();
    match validate_jwt(bearer.token(), &config) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        },
        Err(e) => Err(Error::from(e)),
    }
}

pub async fn validate_admin_jwt_from_bearer(req: ServiceRequest, bearer: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req.app_data::<web::Data<AppConfig>>().expect("AppConfig not found").clone();
    match validate_jwt(bearer.token(), &config) {
        Ok(claims) => {

            req.extensions_mut().insert(claims);
            Ok(req)
        },
        Err(e) => Err(Error::from(e)),
    }
}

