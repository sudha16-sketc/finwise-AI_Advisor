use serde_json::json;
use actix_web::{middleware, web, App, HttpResponse}; // Removed HttpServer
use actix_cors::Cors;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::env;
// Removed dotenvy::dotenv as Shuttle handles secrets differently
use reqwest::Client;

mod routes;
mod stellar;
mod config;
mod db;
mod models;
mod services;
mod utils;

use db::Database;
use models::user::User;

// ... (Your SignupRequest and LoginRequest structs remain the same)

/* =============================
    MAIN (SHUTTLE VERSION)
============================= */

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_actix_web::ShuttleActixWeb {
    // 1. Get secrets from Shuttle instead of .env
    let mongo_uri = secrets.get("MONGODB_URI").expect("MONGODB_URI must be set");
    let session_secret = secrets.get("SESSION_SECRET").expect("SESSION_SECRET must be set");
    let google_client_id = secrets.get("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    
    // Set them as env vars for any sub-modules that still use env::var()
    env::set_var("MONGODB_URI", &mongo_uri);
    env::set_var("GOOGLE_CLIENT_ID", &google_client_id);
    // Add other secrets as needed...

    // 2. Initialize DB
    let db = Database::new().await.unwrap_or_else(|e| {
        panic!("❌ Failed to connect to MongoDB: {e}");
    });
    let db_data = web::Data::new(db);

    let secret_key = Key::from(session_secret.as_bytes());

    // 3. Define the App Configuration
    let config = move |cfg: &mut web::ServiceConfig| {
        let cors = Cors::default()
            .allowed_origin("https://stellar-journey-to-mastery.vercel.app")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization", "Accept"])
            .supports_credentials();

        cfg.service(
            web::scope("")
                .app_data(db_data.clone())
                .wrap(cors)
                .wrap(middleware::Logger::default())
                .wrap(
                    SessionMiddleware::builder(
                        CookieSessionStore::default(),
                        secret_key.clone(),
                    )
                    .cookie_secure(true)
                    .cookie_same_site(actix_web::cookie::SameSite::None)
                    .build()
                )
                .app_data(
                    web::JsonConfig::default()
                        .error_handler(|err, _req| {
                            let response = HttpResponse::BadRequest()
                                .json(json!({ "error": format!("Invalid JSON: {}", err) }));
                            actix_web::error::InternalError::from_response(err, response).into()
                        })
                )
                // AUTH ROUTES
                .route("/api/signup", web::post().to(signup))
                .route("/api/login", web::post().to(login))
                .route("/api/logout", web::post().to(logout))
                .route("/api/check-auth", web::get().to(check_auth))
                .service(
                    web::scope("/api")
                        .route("/balance/{address}", web::get().to(routes::routes::get_balance))
                        .route("/transactions/{address}", web::get().to(routes::routes::get_transactions))
                        .route("/send", web::post().to(routes::routes::send_transaction))
                        .route("/profile", web::get().to(routes::profile::get_profile))  
                        .route("/analyze", web::post().to(routes::analyze::analyze))      
                        .route("/piggy/deposit", web::post().to(routes::piggy::deposit))   
                        .route("/piggy/stats/{user_id}", web::get().to(routes::piggy::get_stats))
                )
                // GENERAL ROUTES
                .route("/", web::get().to(|| async {
                    HttpResponse::Ok().body("FinWise backend running on Shuttle 🚀")
                }))
                .route("/health", web::get().to(routes::health::health_check))
                .route("/auth/google", web::get().to(google_login))
                .route("/auth/google/callback", web::get().to(google_callback))
        );
    };

    // Shuttle handles the HttpServer::new and .bind() automatically
    Ok(config.into())
}

// ... (Your auth handlers remain the same, but update the redirect URLs)