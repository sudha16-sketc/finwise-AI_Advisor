// src/models/metrics.rs
// Metrics struct lives in services::metrics_service to keep DB logic together.
// Re-export here so existing `use crate::models::metrics::Metrics` still compiles.
pub use crate::services::metrics_service::Metrics;