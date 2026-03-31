// src/utils/jwt.rs
//
// JWT creation and verification.
// Tokens expire in 1 hour. JWT_SECRET must be at least 32 characters.

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// User ObjectId hex string.
    pub sub: String,
    /// Expiry as UNIX timestamp (seconds).
    pub exp: usize,
    /// Issued-at as UNIX timestamp.
    pub iat: usize,
}

fn jwt_secret() -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    assert!(
        secret.len() >= 32,
        "JWT_SECRET must be at least 32 characters for production security"
    );
    secret
}

/// Issue a JWT for `user_id` that expires in **1 hour**.
pub fn create_jwt(user_id: &ObjectId) -> Result<String, JwtError> {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_hex(),
        iat: now,
        exp: now + 3600, // 1 hour
    };

    let secret = jwt_secret();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Verify a JWT and return its claims.
/// Returns an error for expired or tampered tokens.
pub fn verify_jwt(token: &str) -> Result<Claims, JwtError> {
    let secret = jwt_secret();
    let mut validation = Validation::default();
    validation.validate_exp = true; // reject expired tokens

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(data.claims)
}