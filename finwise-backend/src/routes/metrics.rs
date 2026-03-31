// src/routes/metrics.rs
// Protected — requires valid JWT.

use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::db::Database;
use crate::services::get_metrics;
use crate::utils::auth_extractor::AuthUser;

pub async fn metrics_handler(
    _auth: AuthUser,
    db: web::Data<Database>,
) -> HttpResponse {
    match get_metrics(&db).await {
        Ok(metrics) => HttpResponse::Ok().json(metrics),
        Err(e) => {
            log::error!("Failed to fetch metrics: {}", e);
            HttpResponse::InternalServerError().json(json!({ "error": e }))
        }
    }
}