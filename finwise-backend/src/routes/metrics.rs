// src/routes/metrics.rs
use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::db::Database;
use crate::services::get_metrics;

pub async fn metrics_handler(db: web::Data<Database>) -> HttpResponse {
    match get_metrics(&db).await {
        Ok(metrics) => HttpResponse::Ok().json(metrics),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}