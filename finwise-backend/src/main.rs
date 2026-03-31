use serde_json::json;
use actix_web::{web, App, HttpServer, HttpResponse, cookie::{Cookie, SameSite}};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use serde::Deserialize;
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::env;
use dotenvy::dotenv;
use reqwest::Client;
use rand::{distributions::Alphanumeric, Rng};

mod routes;
mod stellar;
mod config;
mod db;
mod models;
mod services;
mod utils;
mod app_middleware;

use db::Database;
use models::user::User;
use utils::jwt::create_jwt;
use utils::auth_extractor::AuthUser;
use utils::validation::{validate_email, validate_password, validate_username, validate_wallet};
use app_middleware::security_headers::SecurityHeaders;

// ─── Request DTOs ────────────────────────────────────────────────────────────

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

// ─── Main ─────────────────────────────────────────────────────────────────────

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let _mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
        eprintln!("❌ MONGODB_URI not set in .env");
        std::process::exit(1);
    });

    env::var("JWT_SECRET").unwrap_or_else(|_| {
        eprintln!("❌ JWT_SECRET not set in .env");
        std::process::exit(1);
    });

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let db = Database::new().await.unwrap_or_else(|e| {
        eprintln!("❌ Failed to connect to MongoDB: {e}");
        std::process::exit(1);
    });

    let db_listener = db.clone();
    tokio::spawn(async move {
        services::start_event_listener(db_listener).await;
    });

    let db_data = web::Data::new(db);

    // ── Global rate limiter: 20 req / 60 sec per IP ──────────────────────────
    let global_governor_conf = GovernorConfigBuilder::default()
        .per_second(3)          // ~20 req/min → 1 req / 3 sec burst window
        .burst_size(20)
        .finish()
        .expect("Failed to build global governor config");

    // ── Auth-specific rate limiter: 5 req / 10 sec per IP ────────────────────
    let auth_governor_conf = GovernorConfigBuilder::default()
        .per_second(2)          // 5 req / 10 sec → 1 req / 2 sec
        .burst_size(5)
        .finish()
        .expect("Failed to build auth governor config");

    log::info!("🚀 Starting server at http://0.0.0.0:{}", port);
    log::info!(
        "🌍 Stellar Network: {}",
        env::var("STELLAR_NETWORK").unwrap_or_else(|_| "TESTNET".to_string())
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://finwise-ai-advisor.vercel.app")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization", "Accept"])
            .supports_credentials();

        App::new()
            .app_data(db_data.clone())
            // Security headers on every response
            .wrap(SecurityHeaders)
            .wrap(cors)
            .wrap(Logger::default())
            // Global rate limit
            .wrap(Governor::new(&global_governor_conf))
            .app_data(
                web::JsonConfig::default().error_handler(|err, _req| {
                    let response = HttpResponse::BadRequest()
                        .json(json!({ "error": format!("Invalid JSON: {}", err) }));
                    actix_web::error::InternalError::from_response(err, response).into()
                }),
            )
            // ── Auth routes (stricter rate limit) ────────────────────────────
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/signup")
                            .wrap(Governor::new(&auth_governor_conf))
                            .route(web::post().to(signup)),
                    )
                    .service(
                        web::resource("/login")
                            .wrap(Governor::new(&auth_governor_conf))
                            .route(web::post().to(login)),
                    )
                    .route("/logout", web::post().to(logout))
                    .route("/check-auth", web::get().to(check_auth))
                    // ── Protected routes (JWT required via AuthUser extractor) ─
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

// ─── Auth Handlers ────────────────────────────────────────────────────────────

async fn signup(
    db: web::Data<Database>,
    form: web::Json<SignupRequest>,
) -> HttpResponse {
    // Input validation
    if let Err(e) = validate_username(&form.username) {
        log::warn!("Signup validation failed (username): {}", e);
        return HttpResponse::BadRequest().json(json!({ "message": e }));
    }
    if let Err(e) = validate_email(&form.email) {
        log::warn!("Signup validation failed (email): {}", e);
        return HttpResponse::BadRequest().json(json!({ "message": e }));
    }
    if let Err(e) = validate_password(&form.password) {
        log::warn!("Signup validation failed (password): {}", e);
        return HttpResponse::BadRequest().json(json!({ "message": e }));
    }
    if let Some(wallet) = &form.walletAddress {
        if let Err(e) = validate_wallet(wallet) {
            log::warn!("Signup validation failed (wallet): {}", e);
            return HttpResponse::BadRequest().json(json!({ "message": e }));
        }
    }

    let collection = db.collection::<User>("users");

    let existing = collection
        .find_one(doc! { "email": &form.email })
        .await
        .unwrap();

    if existing.is_some() {
        log::warn!("Signup attempt with existing email: {}", form.email);
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
    };

    if let Some(wallet) = &form.walletAddress {
        services::track_user(&db, wallet).await.ok();
    }

    let insert = collection.insert_one(new_user).await.unwrap();
    let inserted_id = insert.inserted_id.as_object_id().expect("Expected ObjectId");

    let token = match create_jwt(&inserted_id) {
        Ok(t) => t,
        Err(e) => {
            log::error!("❌ Failed to create JWT on signup: {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({ "message": "Failed to create session" }));
        }
    };

    log::info!("✅ New user registered: {}", form.email);

    let cookie = build_auth_cookie(&token);
    HttpResponse::Created()
        .cookie(cookie)
        .json(json!({ "message": "Signup successful" }))
}

async fn login(
    db: web::Data<Database>,
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

                let token = match create_jwt(&user_id) {
                    Ok(t) => t,
                    Err(e) => {
                        log::error!("❌ Failed to create JWT on login: {}", e);
                        return HttpResponse::InternalServerError()
                            .json(json!({ "message": "Failed to create session" }));
                    }
                };

                log::info!("✅ Successful login: {}", form.email);
                let cookie = build_auth_cookie(&token);
                return HttpResponse::Ok()
                    .cookie(cookie)
                    .json(json!({ "message": "Login successful" }));
            }
        }
    }

    log::warn!("⚠️ Failed login attempt for email: {}", form.email);
    HttpResponse::BadRequest().json(json!({ "message": "Invalid credentials" }))
}

async fn logout() -> HttpResponse {
    // Expire the HttpOnly cookie immediately
    let expired = Cookie::build("auth_token", "")
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::seconds(0))
        .finish();

    HttpResponse::Ok()
        .cookie(expired)
        .json(json!({ "message": "Logout successful" }))
}

async fn check_auth(
    db: web::Data<Database>,
    auth: AuthUser,
) -> HttpResponse {
    let collection = db.collection::<User>("users");

    let _ = collection
        .update_one(
            doc! { "_id": auth.0 },
            doc! {
                "$set": { "last_active": BsonDateTime::now() },
                "$inc": { "total_actions": 1i64 }
            },
        )
        .await;

    let user = collection
        .find_one(doc! { "_id": auth.0 })
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

    log::warn!("⚠️ check-auth: user not found for id {:?}", auth.0);
    HttpResponse::Unauthorized().json(json!({ "authenticated": false }))
}

// ─── Google OAuth ─────────────────────────────────────────────────────────────

async fn google_login() -> HttpResponse {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set");
    let redirect_uri = "https://finwise-ai-advisor.onrender.com/auth/google/callback";

    // Generate a random CSRF state token
    let state: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let google_auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth\
         ?client_id={}&redirect_uri={}&response_type=code\
         &scope=openid%20email%20profile&access_type=offline&prompt=consent\
         &state={}",
        client_id, redirect_uri, state
    );

    // Store state in a short-lived HttpOnly cookie for validation in callback
    let state_cookie = Cookie::build("oauth_state", state)
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax) // Lax required so cookie survives the redirect back
        .max_age(actix_web::cookie::time::Duration::minutes(10))
        .finish();

    HttpResponse::Found()
        .cookie(state_cookie)
        .append_header(("Location", google_auth_url))
        .finish()
}

async fn google_callback(
    db: web::Data<Database>,
    query: web::Query<std::collections::HashMap<String, String>>,
    req: actix_web::HttpRequest,
) -> HttpResponse {
    // ── CSRF state validation ─────────────────────────────────────────────────
    let expected_state = req.cookie("oauth_state").map(|c| c.value().to_string());
    let received_state = query.get("state").cloned();

    match (expected_state, received_state) {
        (Some(expected), Some(received)) if expected == received => {} // OK
        _ => {
            log::warn!("⚠️ OAuth CSRF state mismatch — possible CSRF attack");
            return HttpResponse::Found()
                .append_header((
                    "Location",
                    "https://finwise-ai-advisor.vercel.app/login?error=csrf_mismatch",
                ))
                .finish();
        }
    }

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
                return redirect_error("token_parse_failed");
            }
        },
        Err(e) => {
            log::error!("❌ Token exchange request failed: {}", e);
            return redirect_error("token_request_failed");
        }
    };

    if let Some(err) = token_res.get("error") {
        log::error!(
            "❌ Google token error: {} - {:?}",
            err,
            token_res.get("error_description")
        );
        return redirect_error("token_failed");
    }

    let access_token = match token_res["access_token"].as_str() {
        Some(t) => t.to_string(),
        None => {
            log::error!("❌ No access_token in response: {:?}", token_res);
            return redirect_error("no_access_token");
        }
    };

    let user_info = match client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&access_token)
        .send()
        .await
    {
        Ok(res) => match res.json::<serde_json::Value>().await {
            Ok(json) => json,
            Err(e) => {
                log::error!("❌ Failed to parse userinfo response: {}", e);
                return redirect_error("userinfo_parse_failed");
            }
        },
        Err(e) => {
            log::error!("❌ Userinfo request failed: {}", e);
            return redirect_error("userinfo_failed");
        }
    };

    let email = match user_info["email"].as_str() {
        Some(e) => e.to_string(),
        None => {
            log::error!("❌ No email in userinfo response");
            return redirect_error("no_email");
        }
    };

    let username = email.split('@').next().unwrap_or("user").to_string();
    let users = db.collection::<User>("users");

    let existing = match users.find_one(doc! { "email": &email }).await {
        Ok(result) => result,
        Err(e) => {
            log::error!("❌ MongoDB find_one failed: {}", e);
            return redirect_error("db_error");
        }
    };

    let user_id: ObjectId = if let Some(user) = existing {
        match user.id {
            Some(id) => id,
            None => {
                log::error!("❌ Existing user has no ObjectId");
                return redirect_error("user_id_missing");
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
        };

        match users.insert_one(new_user).await {
            Ok(insert) => match insert.inserted_id.as_object_id() {
                Some(id) => id,
                None => {
                    log::error!("❌ Inserted ID is not an ObjectId");
                    return redirect_error("insert_id_error");
                }
            },
            Err(e) => {
                log::error!("❌ Failed to insert new user: {:?}", e);
                return redirect_error("insert_failed");
            }
        }
    };

    let _ = users
        .update_one(
            doc! { "_id": user_id },
            doc! {
                "$set": { "last_active": BsonDateTime::now() },
                "$inc": { "total_actions": 1i64 }
            },
        )
        .await;

    let token = match create_jwt(&user_id) {
        Ok(t) => t,
        Err(e) => {
            log::error!("❌ Failed to create JWT: {}", e);
            return redirect_error("jwt_failed");
        }
    };

    log::info!("✅ Google OAuth success for {}", email);

    // Clear the CSRF state cookie
    let clear_state = Cookie::build("oauth_state", "")
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .max_age(actix_web::cookie::time::Duration::seconds(0))
        .finish();

    // Set JWT in HttpOnly, Secure cookie — NOT in the URL
    let auth_cookie = build_auth_cookie(&token);

    HttpResponse::Found()
        .cookie(clear_state)
        .cookie(auth_cookie)
        .append_header(("Location", "https://finwise-ai-advisor.vercel.app/dashboard"))
        .finish()
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Build the HttpOnly auth cookie for JWT storage.
fn build_auth_cookie(token: &str) -> Cookie<'static> {
    Cookie::build("auth_token", token.to_owned())
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::hours(1))
        .finish()
}

/// Redirect to the frontend login page with a specific error code.
fn redirect_error(code: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((
            "Location",
            format!("https://finwise-ai-advisor.vercel.app/login?error={}", code),
        ))
        .finish()
}