use actix_web::{middleware, web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;

mod routes;
mod stellar;

/**
 * Main entry point for the Stellar dApp backend
 * Starts an HTTP server using Actix-web
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get server configuration from environment or use defaults
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    log::info!("Starting Stellar dApp backend server at http://{}", bind_address);
    log::info!("Connected to Stellar {} network", 
        env::var("STELLAR_NETWORK").unwrap_or_else(|_| "TESTNET".to_string())
    );

    // Start HTTP server
    HttpServer::new(|| {
        // Configure CORS to allow requests from frontend
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // Enable CORS
            .wrap(cors)
            // Enable logger middleware
            .wrap(middleware::Logger::default())
            // Register API routes
            .service(
                web::scope("/api")
                    .route("/balance/{address}", web::get().to(routes::get_balance))
                    .route("/transactions/{address}", web::get().to(routes::get_transactions))
                    .route("/send", web::post().to(routes::send_transaction))
            )
            // Health check endpoint
            .route("/health", web::get().to(routes::health_check))
    })
    .bind(&bind_address)?
    .run()
    .await
}