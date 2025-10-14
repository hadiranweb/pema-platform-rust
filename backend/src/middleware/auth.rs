use actix_web::{
    dev::{forward_ready, Service, ServiceFactory, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures::future::{self, LocalBoxFuture, Ready};
use std::{cell::RefCell, rc::Rc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub exp: usize,
}

pub struct JwtMiddleware {
    pub secret: String,
}

impl<S, B>
    Transform<S, ServiceRequest>
    for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(JwtMiddlewareService { service: Rc::new(RefCell::new(service)), secret: self.secret.clone() }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<RefCell<S>>,
    secret: String,
}

impl<S, B>
    Service<ServiceRequest>
    for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let (http_req, payload) = req.into_parts();
        let auth_header = http_req.headers().get("Authorization");

        let secret = self.secret.clone();
        let srv = self.service.clone();

        Box::pin(async move {
            if let Some(header_value) = auth_header {
                if let Ok(header_str) = header_value.to_str() {
                    if header_str.starts_with("Bearer ") {
                        let token = &header_str[7..];
                        let validation = Validation::new(jsonwebtoken::Algorithm::HS512);
                        match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
                            Ok(token_data) => {
                                http_req.extensions_mut().insert(token_data.claims);
                                let req = ServiceRequest::from_parts(http_req, payload);
                                let res = srv.call(req).await?;
                                return Ok(res.map_into_left_body());
                            }
                            Err(_) => {
                                let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                                return Ok(ServiceResponse::new(http_req, res));
                            }
                        }
                    }
                }
            }
            let res = HttpResponse::Unauthorized().finish().map_into_right_body();
            Ok(ServiceResponse::new(http_req, res))
        })
    }
}

pub fn create_jwt(user_id: Uuid, role: String, secret: &str, expires_in: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(expires_in))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        role,
        exp: expiration as usize,
    };
    let header = Header::new(jsonwebtoken::Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
}

