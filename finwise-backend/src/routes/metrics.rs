// src/routes/metrics.rs
use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::db::Database;
use crate::services::get_metrics;


pub async fn metrics_handler(db: web::Data<Database>) -> Result<HttpResponse> {
    match get_metrics(&db).await {
        Ok(metrics) => Ok(HttpResponse::Ok().json(metrics)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({ "error": e }))),
    }
}

