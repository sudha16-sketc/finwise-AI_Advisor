use actix_web::{FromRequest, HttpRequest, dev::Payload, Error, error::ErrorUnauthorized};
use futures::future::{ready, Ready};
use mongodb::bson::oid::ObjectId;
use crate::utils::jwt::verify_jwt;

pub struct AuthUser(pub ObjectId);

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        match token {
            Some(t) => match verify_jwt(t) {
                Ok(claims) => match ObjectId::parse_str(&claims.sub) {
                    Ok(id) => ready(Ok(AuthUser(id))),
                    Err(_) => ready(Err(ErrorUnauthorized("Invalid user id in token"))),
                },
                Err(_) => ready(Err(ErrorUnauthorized("Invalid or expired token"))),
            },
            None => ready(Err(ErrorUnauthorized("Missing Authorization header"))),
        }
    }
}