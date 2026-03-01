use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("MongoDB error: {0}")]
    Database(#[from] mongodb::error::Error),

    #[error("Ollama API error: {0}")]
    OllamaApi(String),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Already deposited today")]
    AlreadyDepositedToday,

    #[error("Not found: {0}")]
    NotFound(String),
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Validation(msg) | AppError::NotFound(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": msg,
                    "success": false
                }))
            }
            AppError::AlreadyDepositedToday => {
                HttpResponse::Conflict().json(serde_json::json!({
                    "error": "You have already made a deposit today. Come back tomorrow!",
                    "success": false
                }))
            }
            _ => {
                log::error!("Internal error: {}", self);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "An internal error occurred",
                    "success": false
                }))
            }
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;