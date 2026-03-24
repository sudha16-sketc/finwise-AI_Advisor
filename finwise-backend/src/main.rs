// src/main.rs
use serde_json::json;
use actix_web::{middleware, web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::{Key, SameSite};
use serde::Deserialize;
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::env;
use dotenvy::dotenv;
use reqwest::Client;
use rand::Rng;

mod routes;
mod stellar;
mod config;
mod db;
mod models;
mod services;
mod utils;

use db::Database;
use models::user::User;

#[derive(Deserialize)]
struct SignupRequest {
    username: String,
    email: String,
    password: String,
    #[allow(non_snake_case)]
    walletAddress: Option<String>,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct TokenQuery {
    token: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let _mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
        eprintln!("❌ MONGODB_URI not set in .env");
        std::process::exit(1);
    });

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    let bind_address = format!("0.0.0.0:{}", port);

    println!("🚀 Server running at http://{}", bind_address);
    log::info!("🚀 Starting server at http://{}", bind_address);
    log::info!(
        "🌍 Stellar Network: {}",
        env::var("STELLAR_NETWORK").unwrap_or_else(|_| "TESTNET".to_string())
    );

    let db = Database::new().await.unwrap_or_else(|e| {
        eprintln!("❌ Failed to connect to MongoDB: {e}");
        std::process::exit(1);
    });

    let db_listener = db.clone();
    tokio::spawn(async move {
        services::start_event_listener(db_listener).await;
    });

    let db_data = web::Data::new(db);

    let secret_key = Key::from(
        env::var("SESSION_SECRET")
            .expect("SESSION_SECRET must be set")
            .as_bytes(),
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://finwise-ai-advisor.vercel.app")
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
                .cookie_name("finwise_session".to_string())
                .cookie_secure(true)
                .cookie_same_site(SameSite::None)
                .cookie_domain(Some("finwise-ai-advisor.onrender.com".to_string())) 
                .build()
            )
            .app_data(
                web::JsonConfig::default().error_handler(|err, _req| {
                    let response = HttpResponse::BadRequest()
                        .json(json!({ "error": format!("Invalid JSON: {}", err) }));
                    actix_web::error::InternalError::from_response(err, response).into()
                }),
            )
            .route("/api/signup", web::post().to(signup))
            .route("/api/login", web::post().to(login))
            .route("/api/logout", web::post().to(logout))
            .route("/api/check-auth", web::get().to(check_auth))
            .route("/api/verify-token", web::get().to(verify_token))
            .service(
                web::scope("/api")
                    .route("/balance/{address}", web::get().to(routes::routes::get_balance))
                    .route("/transactions/{address}", web::get().to(routes::routes::get_transactions))
                    .route("/send", web::post().to(routes::routes::send_transaction))
                    .route("/profile", web::get().to(routes::profile::get_profile))
                    .route("/analyze", web::post().to(routes::analyze::analyze))
                    .route("/metrics", web::get().to(routes::metrics::metrics_handler))
                    .route("/track-user", web::post().to(routes::user::track_user_handler)),
            )
            .route("/", web::get().to(|| async {
                HttpResponse::Ok().body("FinWise backend running 🚀")
            }))
            .route("/health", web::get().to(routes::health::health_check))
            .route("/auth/google", web::get().to(google_login))
            .route("/auth/google/callback", web::get().to(google_callback))
    })
    .bind(("0.0.0.0", port))?
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
            .json(json!({ "message": "Email already exists" }));
    }

    let hashed = hash(&form.password, DEFAULT_COST).unwrap();

    let new_user = User {
        id: None,
        username: form.username.clone(),
        email: form.email.clone(),
        password: hashed,
        wallet_address: form.walletAddress.clone(),
        created_at: BsonDateTime::now(),
        last_active: Some(BsonDateTime::now()),
        total_actions: 0,
        oauth_token: None,
    };

    if let Some(wallet) = &form.walletAddress {
        services::track_user(&db, wallet).await.ok();
    }

    let insert = collection.insert_one(new_user).await.unwrap();
    let inserted_id = insert
        .inserted_id
        .as_object_id()
        .expect("Expected ObjectId");

    session.insert("user_id", inserted_id).unwrap();

    HttpResponse::Created().json(json!({ "message": "Signup successful" }))
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
                let _ = collection
                    .update_one(
                        doc! { "_id": user_id },
                        doc! {
                            "$set": { "last_active": BsonDateTime::now() },
                            "$inc": { "total_actions": 1i64 }
                        },
                    )
                    .await;

                session.insert("user_id", user_id).unwrap();
            }
            return HttpResponse::Ok().json(json!({ "message": "Login successful" }));
        }
    }

    HttpResponse::BadRequest().json(json!({ "message": "Invalid credentials" }))
}

async fn logout(session: actix_session::Session) -> HttpResponse {
    session.purge();
    HttpResponse::Ok().json(json!({ "message": "Logout successful" }))
}

async fn check_auth(
    db: web::Data<Database>,
    session: actix_session::Session,
) -> HttpResponse {
    let user_id = match session.get::<ObjectId>("user_id") {
        Ok(Some(id)) => id,
        _ => {
            return HttpResponse::Ok().json(json!({ "authenticated": false }));
        }
    };

    let collection = db.collection::<User>("users");

    let _ = collection
        .update_one(
            doc! { "_id": user_id },
            doc! {
                "$set": { "last_active": BsonDateTime::now() },
                "$inc": { "total_actions": 1i64 }
            },
        )
        .await;

    let user = collection
        .find_one(doc! { "_id": user_id })
        .await
        .unwrap();

    if let Some(user) = user {
        return HttpResponse::Ok().json(json!({
            "authenticated": true,
            "user": {
                "username": user.username,
                "email": user.email,
                "wallet_address": user.wallet_address
            }
        }));
    }

    HttpResponse::Ok().json(json!({ "authenticated": false }))
}

/// Exchange a one-time OAuth token for a session cookie.
/// Called by the frontend /auth/callback page with credentials: "include".
/// This is the correct way to set a cross-site session cookie — via a direct
/// credentialed fetch, NOT via a redirect (browsers block cookies set during redirects).
async fn verify_token(
    db: web::Data<Database>,
    session: actix_session::Session,
    query: web::Query<TokenQuery>,
) -> HttpResponse {
    let users = db.collection::<User>("users");

    let user = match users.find_one(doc! { "oauth_token": &query.token }).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            log::warn!("⚠️ verify_token: token not found or already used");
            return HttpResponse::Unauthorized().json(json!({ "authenticated": false }));
        }
        Err(e) => {
            log::error!("❌ verify_token DB error: {}", e);
            return HttpResponse::InternalServerError().json(json!({ "authenticated": false }));
        }
    };

    let user_id = match user.id {
        Some(id) => id,
        None => {
            log::error!("❌ verify_token: user has no ObjectId");
            return HttpResponse::InternalServerError().json(json!({ "authenticated": false }));
        }
    };

    // Clear the one-time token immediately so it cannot be reused
    let _ = users.update_one(
        doc! { "_id": user_id },
        doc! { "$unset": { "oauth_token": "" } },
    ).await;

    // Set session — this response goes directly back to the browser via a
    // credentialed fetch, so Set-Cookie works correctly cross-site
    if let Err(e) = session.insert("user_id", user_id) {
        log::error!("❌ verify_token session insert failed: {}", e);
        return HttpResponse::InternalServerError().json(json!({ "authenticated": false }));
    }

    log::info!("✅ verify_token success for user {}", user_id);

    HttpResponse::Ok().json(json!({
        "authenticated": true,
        "user": {
            "username": user.username,
            "email": user.email,
            "wallet_address": user.wallet_address
        }
    }))
}

async fn google_login() -> HttpResponse {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set");
    let redirect_uri = "https://finwise-ai-advisor.onrender.com/auth/google/callback";

    let google_auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth\
         ?client_id={}&redirect_uri={}&response_type=code\
         &scope=openid%20email%20profile&access_type=offline&prompt=consent",
        client_id, redirect_uri
    );

    HttpResponse::Found()
        .append_header(("Location", google_auth_url))
        .finish()
}

async fn google_callback(
    db: web::Data<Database>,
    _session: actix_session::Session,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    if let Some(error) = query.get("error") {
        log::error!("❌ Google OAuth returned error: {}", error);
        return HttpResponse::Found()
            .append_header((
                "Location",
                "https://finwise-ai-advisor.vercel.app/login?error=oauth_denied",
            ))
            .finish();
    }

    let code = match query.get("code") {
        Some(c) => c,
        None => {
            log::error!("❌ No code in Google callback");
            return HttpResponse::Found()
                .append_header((
                    "Location",
                    "https://finwise-ai-advisor.vercel.app/login?error=no_code",
                ))
                .finish();
        }
    };

    let client_id = env::var("GOOGLE_CLIENT_ID").unwrap();
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").unwrap();
    let client = Client::new();

    let token_response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            (
                "redirect_uri",
                "https://finwise-ai-advisor.onrender.com/auth/google/callback",
            ),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await;

    let token_res = match token_response {
        Ok(res) => match res.json::<serde_json::Value>().await {
            Ok(json) => json,
            Err(e) => {
                log::error!("❌ Failed to parse token response: {}", e);
                return HttpResponse::Found()
                    .append_header((
                        "Location",
                        "https://finwise-ai-advisor.vercel.app/login?error=token_parse_failed",
                    ))
                    .finish();
            }
        },
        Err(e) => {
            log::error!("❌ Token exchange request failed: {}", e);
            return HttpResponse::Found()
                .append_header((
                    "Location",
                    "https://finwise-ai-advisor.vercel.app/login?error=token_request_failed",
                ))
                .finish();
        }
    };

    if let Some(err) = token_res.get("error") {
        log::error!(
            "❌ Google token error: {} - {:?}",
            err,
            token_res.get("error_description")
        );
        return HttpResponse::Found()
            .append_header((
                "Location",
                "https://finwise-ai-advisor.vercel.app/login?error=token_failed",
            ))
            .finish();
    }

    let access_token = match token_res["access_token"].as_str() {
        Some(t) => t.to_string(),
        None => {
            log::error!("❌ No access_token in response: {:?}", token_res);
            return HttpResponse::Found()
                .append_header((
                    "Location",
                    "https://finwise-ai-advisor.vercel.app/login?error=no_access_token",
                ))
                .finish();
        }
    };

    let userinfo_response = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&access_token)
        .send()
        .await;

    let user_info = match userinfo_response {
        Ok(res) => match res.json::<serde_json::Value>().await {
            Ok(json) => json,
            Err(e) => {
                log::error!("❌ Failed to parse userinfo response: {}", e);
                return HttpResponse::Found()
                    .append_header((
                        "Location",
                        "https://finwise-ai-advisor.vercel.app/login?error=userinfo_parse_failed",
                    ))
                    .finish();
            }
        },
        Err(e) => {
            log::error!("❌ Userinfo request failed: {}", e);
            return HttpResponse::Found()
                .append_header((
                    "Location",
                    "https://finwise-ai-advisor.vercel.app/login?error=userinfo_failed",
                ))
                .finish();
        }
    };

    let email = match user_info["email"].as_str() {
        Some(e) => e.to_string(),
        None => {
            log::error!("❌ No email in userinfo response: {:?}", user_info);
            return HttpResponse::Found()
                .append_header((
                    "Location",
                    "https://finwise-ai-advisor.vercel.app/login?error=no_email",
                ))
                .finish();
        }
    };

    let username = email.split('@').next().unwrap_or("user").to_string();
    let users = db.collection::<User>("users");

    let existing = match users.find_one(doc! { "email": &email }).await {
        Ok(result) => result,
        Err(e) => {
            log::error!("❌ MongoDB find_one failed: {}", e);
            return HttpResponse::Found()
                .append_header((
                    "Location",
                    "https://finwise-ai-advisor.vercel.app/login?error=db_error",
                ))
                .finish();
        }
    };

    let user_id = if let Some(user) = existing {
        match user.id {
            Some(id) => id,
            None => {
                log::error!("❌ Existing user has no ObjectId");
                return HttpResponse::Found()
                    .append_header((
                        "Location",
                        "https://finwise-ai-advisor.vercel.app/login?error=user_id_missing",
                    ))
                    .finish();
            }
        }
    } else {
        let new_user = User {
            id: None,
            username,
            email: email.clone(),
            password: "".into(),
            wallet_address: None,
            created_at: BsonDateTime::now(),
            last_active: Some(BsonDateTime::now()),
            total_actions: 0,
            oauth_token: None,
        };

        match users.insert_one(new_user).await {
            Ok(insert) => match insert.inserted_id.as_object_id() {
                Some(id) => id,
                None => {
                    log::error!("❌ Inserted ID is not an ObjectId");
                    return HttpResponse::Found()
                        .append_header((
                            "Location",
                            "https://finwise-ai-advisor.vercel.app/login?error=insert_id_error",
                        ))
                        .finish();
                }
            },
            Err(e) => {
                log::error!("❌ Failed to insert new user: {:?}", e);
                return HttpResponse::Found()
                    .append_header((
                        "Location",
                        "https://finwise-ai-advisor.vercel.app/login?error=insert_failed",
                    ))
                    .finish();
            }
        }
    };

    // Generate a one-time token to hand off to the frontend.
    // The frontend /auth/callback page will call /api/verify-token?token=...
    // with credentials: "include" — this properly sets the session cookie
    // cross-site unlike a redirect which browsers block cookies on.
    let token: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let _ = users.update_one(
        doc! { "_id": user_id },
        doc! {
            "$set": {
                "last_active": BsonDateTime::now(),
                "oauth_token": &token
            },
            "$inc": { "total_actions": 1i64 }
        },
    ).await;

    log::info!("✅ Google OAuth callback done for {}, redirecting to frontend with token", email);

    HttpResponse::Found()
        .append_header((
            "Location",
            format!("https://finwise-ai-advisor.vercel.app/auth/callback?token={}", token),
        ))
        .finish()
}