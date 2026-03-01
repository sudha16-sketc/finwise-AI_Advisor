use std::env;

/// Application configuration loaded from environment variables
#[derive(Clone, Debug)]
pub struct Config {
    pub mongodb_uri: String,
    pub gemini_api_key: String,
    pub gemini_api_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            mongodb_uri: env::var("MONGODB_URI")?,
            gemini_api_key: env::var("GEMINI_API_KEY")?,
            gemini_api_url: env::var("GEMINI_API_URL").unwrap_or_else(|_| {
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent".to_string()
            }),
        })
    }
}