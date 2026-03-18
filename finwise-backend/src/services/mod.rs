// src/services/mod.rs                                                                  
pub mod nlp_service;
pub mod piggy_service;
pub mod ollama_service;
pub mod user_service;
pub mod metrics_service;
pub mod event_listener;

pub use ollama_service::AiService;
pub use nlp_service::parse_financial_text;
pub use piggy_service::PiggyService;
pub use user_service::{track_user, log_transaction};
pub use metrics_service::get_metrics;
pub use event_listener::start_event_listener;
