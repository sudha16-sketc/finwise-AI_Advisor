// src/utils/auth_extractor.rs
//
// Extracts and validates JWT from:
//   1. Authorization: Bearer <token>  header
//   2. auth_token HttpOnly cookie (set during login/OAuth)
//
// Logs invalid JWT usage for security monitoring.

use actix_web::{FromRequest, HttpRequest, dev::Payload, Error, error::ErrorUnauthorized};
use futures::future::{ready, Ready};
use mongodb::bson::oid::ObjectId;
use crate::utils::jwt::verify_jwt;

pub struct AuthUser(pub ObjectId);

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // 1. Try Bearer header first (API clients)
        let header_token = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|t| t.to_string());

        // 2. Fall back to HttpOnly cookie (browser clients)
        let cookie_token = req
            .cookie("auth_token")
            .map(|c| c.value().to_string());

        let token = match header_token.or(cookie_token) {
            Some(t) => t,
            None => {
                log::warn!(
                    "⚠️ Unauthorized access attempt — no credentials on {} {}",
                    req.method(),
                    req.path()
                );
                return ready(Err(ErrorUnauthorized("Missing credentials")));
            }
        };

        match verify_jwt(&token) {
            Ok(claims) => match ObjectId::parse_str(&claims.sub) {
                Ok(id) => ready(Ok(AuthUser(id))),
                Err(_) => {
                    log::warn!(
                        "⚠️ Invalid user id in JWT on {} {}",
                        req.method(),
                        req.path()
                    );
                    ready(Err(ErrorUnauthorized("Invalid user id in token")))
                }
            },
            Err(e) => {
                log::warn!(
                    "⚠️ Invalid/expired JWT on {} {} — {}",
                    req.method(),
                    req.path(),
                    e
                );
                ready(Err(ErrorUnauthorized("Invalid or expired token")))
            }
        }
    }
}