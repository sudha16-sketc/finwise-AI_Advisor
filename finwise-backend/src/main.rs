use serde_json::json;
use actix_web::{middleware, web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::env;
use dotenvy::dotenv;
use reqwest::Client;
use actix_session::config::PersistentSession;
use std::time::Duration;

mod routes;
mod stellar;
mod config;
mod db;
mod models;
mod services;
mod utils;

use db::Database;
use models::user::User; // make sure you have this model

/* =============================
   REQUEST STRUCTS
============================= */

#[derive(Deserialize)]
struct SignupRequest {
    username: String,
    email: String,
    password: String,
    walletAddress: Option<String>,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

/* =============================
   MAIN
============================= */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
        eprintln!("❌ MONGODB_URI not set in .env");
        std::process::exit(1);
    });

    
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    println!("🚀 Server running at http://{}", bind_address);
    log::info!("🚀 Starting server at http://{}", bind_address);
    log::info!("🌍 Stellar Network: {}",
        env::var("STELLAR_NETWORK").unwrap_or_else(|_| "TESTNET".to_string())
    );

    let db = Database::new().await.unwrap_or_else(|e| {
        eprintln!("❌ Failed to connect to MongoDB: {e}");
        std::process::exit(1);
    });

    let db_data = web::Data::new(db);

    let secret_key = Key::from(
        env::var("SESSION_SECRET")
            .expect("SESSION_SECRET must be set")
            .as_bytes()
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://stellar-journey-to-mastery.vercel.app")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization", "Accept"])
            .supports_credentials();
        App::new()
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

            /* =============================
               AUTH ROUTES
            ============================== */

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
            /* =============================
               EXISTING ROUTES
            ============================== */

            .route("/health", web::get().to(routes::health::health_check))
            .route("/auth/google", web::get().to(google_login))
            .route("/auth/google/callback", web::get().to(google_callback))
            
    })
    .bind(bind_address)?
    .run()
    .await
}

/* =============================
   AUTH HANDLERS
============================= */

async fn signup(
    db: web::Data<Database>,
    session: actix_session::Session,
    form: web::Json<SignupRequest>,
) -> HttpResponse {
    let collection = db.collection::<User>("users");

    let existing = collection
        .find_one(doc! { "email": &form.email })
        .await
        .unwrap();

    if existing.is_some() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({ "message": "Email already exists" }));
    }

    let hashed = hash(&form.password, DEFAULT_COST).unwrap();

    let new_user = User {
        id: None,
        username: form.username.clone(),
        email: form.email.clone(),
        password: hashed,
        wallet_address: form.walletAddress.clone(),
        created_at: Utc::now(),
    };

    let insert = collection.insert_one(new_user).await.unwrap();
    let inserted_id = insert.inserted_id
        .as_object_id()
        .expect("Expected ObjectId");

    session.insert("user_id", inserted_id).unwrap();

    

    HttpResponse::Created()
        .json(serde_json::json!({ "message": "Signup successful" }))
}

async fn login(
    db: web::Data<Database>,
    session: actix_session::Session,
    form: web::Json<LoginRequest>,
) -> HttpResponse {
    let collection = db.collection::<User>("users");

    let user = collection
        .find_one(doc! { "email": &form.email })
        .await
        .unwrap();

    if let Some(user) = user {
        if verify(&form.password, &user.password).unwrap_or(false) {
            if let Some(user_id) = user.id {
                session.insert("user_id", user_id).unwrap();
            }
            return HttpResponse::Ok()
                .json(serde_json::json!({ "message": "Login successful" }));
        }
    }

    HttpResponse::BadRequest()
        .json(serde_json::json!({ "message": "Invalid credentials" }))
}

async fn logout(session: actix_session::Session) -> HttpResponse {
    session.purge();
    HttpResponse::Ok()
        .json(serde_json::json!({ "message": "Logout successful" }))
}

async fn check_auth(
    db: web::Data<Database>,
    session: actix_session::Session,
) -> HttpResponse {
    let user_id = match session.get::<ObjectId>("user_id") {
        Ok(Some(id)) => id,
        _ => {
            return HttpResponse::Ok()
                .json(serde_json::json!({ "authenticated": false }))
        }
    };

    let collection = db.collection::<User>("users");

    let user = collection
        .find_one(doc! { "_id": user_id })
        .await
        .unwrap();

    if let Some(user) = user {
        return HttpResponse::Ok().json(serde_json::json!({
            "authenticated": true,
            "user": {
                "username": user.username,
                "email": user.email,
                "wallet_address": user.wallet_address
            }
        }));
    }

    HttpResponse::Ok()
        .json(serde_json::json!({ "authenticated": false }))
}

async fn google_login() -> HttpResponse {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set");

    let redirect_uri = "https://finwise-backend.up.railway.app/auth/google/callback";

    let google_auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&access_type=offline&prompt=consent",
        client_id,
        redirect_uri
    );

    HttpResponse::Found()
        .append_header(("Location", google_auth_url))
        .finish()
}



async fn google_callback(
    db: web::Data<Database>,
    session: actix_session::Session,
    query: web::Query<std::collections::HashMap<String, String>>
) -> HttpResponse {

    let code = match query.get("code") {
        Some(c) => c,
        None => return HttpResponse::BadRequest().body("No code found")
    };

    let client_id = env::var("GOOGLE_CLIENT_ID").unwrap();
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").unwrap();

    let client = Client::new();

    // Exchange code for token
    let token_res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("redirect_uri", &"https://finwise-backend.up.railway.app/auth/google/callback".to_string()),
            ("grant_type", &"authorization_code".to_string()),
        ])
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let access_token = token_res["access_token"]
        .as_str()
        .unwrap();

    // Get user info
    let user_info = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let email = user_info["email"].as_str().unwrap().to_string();

    // 🔥 Derive username from email
    let username = email.split('@').next().unwrap().to_string();

    let users = db.collection::<User>("users");

    let existing = users
        .find_one(doc! { "email": &email })
        .await
        .unwrap();

    let user_id = if let Some(user) = existing {
        user.id.unwrap()
    } else {
        let new_user = User {
            id: None,
            username,
            email: email.clone(),
            password: "".into(), // no password for Google users
            wallet_address: None,
            created_at: Utc::now(),
        };

        let insert = users.insert_one(new_user).await.unwrap();
        insert.inserted_id.as_object_id().unwrap()
    };

    session.insert("user_id", user_id).unwrap();

    HttpResponse::Found()
        .append_header(("Location", "http://stellar-journey-to-mastery.vercel.app/dashboard"))
        .finish()
}



