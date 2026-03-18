// src/routes/user.rs
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use crate::db::Database;
use crate::services::track_user;

#[derive(Deserialize)]
pub struct TrackRequest {
    pub wallet_address: String,
}

pub async fn track_user_handler(
    db: web::Data<Database>,
    req: web::Json<TrackRequest>,
) -> Result<HttpResponse> {
    match track_user(&db, &req.wallet_address).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({ "error": e }))),
    }
}

