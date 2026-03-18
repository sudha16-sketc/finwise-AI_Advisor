//  src/routes/routes.rs
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::stellar;

/**
 * API Routes
 * Defines all REST API endpoints for the Stellar dApp backend
 */

/**
 * Response structure for balance endpoint
 */
#[derive(Serialize)]
struct BalanceResponse {
    balance: String,
    address: String,
}

/**
 * Response structure for transactions endpoint
 */
#[derive(Serialize)]
struct TransactionsResponse {
    transactions: Vec<stellar::Transaction>,
    count: usize,
}

/**
 * Request structure for send transaction endpoint
 */
#[derive(Deserialize)]
pub struct SendTransactionRequest {
    xdr: String, // Signed transaction XDR
}

/**
 * Response structure for send transaction endpoint
 */
#[derive(Serialize)]
struct SendTransactionResponse {
    hash: String,
    ledger: i64,
    success: bool,
    message: String,
}

/**
 * Error response structure
 */
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

/**
 * GET /api/balance/{address}
 * Fetch XLM balance for a given Stellar address
 */
pub async fn get_balance(address: web::Path<String>) -> impl Responder {
    let address = address.into_inner();

    log::info!("GET /api/balance/{}", address);

    // Validate address format
    if !stellar::is_valid_address(&address) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "invalid_address".to_string(),
            message: "Invalid Stellar address format".to_string(),
        });
    }

    // Fetch balance from Stellar network
    match stellar::fetch_account_balance(&address).await {
        Ok(balance) => HttpResponse::Ok().json(BalanceResponse {
            balance,
            address: address.clone(),
        }),
        Err(e) => {
            log::error!("Failed to fetch balance for {}: {}", address, e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "fetch_failed".to_string(),
                message: format!("Failed to fetch balance: {}", e),
            })
        }
    }
}

/**
 * GET /api/transactions/{address}
 * Fetch transaction history for a given Stellar address
 */
pub async fn get_transactions(
    address: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let address = address.into_inner();
    let limit = query
        .get("limit")
        .and_then(|l| l.parse::<i32>().ok());

    log::info!("GET /api/transactions/{}", address);

    // Validate address format
    if !stellar::is_valid_address(&address) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "invalid_address".to_string(),
            message: "Invalid Stellar address format".to_string(),
        });
    }

    // Fetch transaction history
    match stellar::fetch_transaction_history(&address, limit).await {
        Ok(transactions) => {
            let count = transactions.len();
            HttpResponse::Ok().json(TransactionsResponse {
                transactions,
                count,
            })
        }
        Err(e) => {
            log::error!("Failed to fetch transactions for {}: {}", address, e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "fetch_failed".to_string(),
                message: format!("Failed to fetch transactions: {}", e),
            })
        }
    }
}

/**
 * POST /api/send
 * Submit a signed transaction to the Stellar network
 * 
 * Body: { "xdr": "signed transaction XDR" }
 */
pub async fn send_transaction(req: web::Json<SendTransactionRequest>) -> impl Responder {
    log::info!("POST /api/send - Submitting transaction");

    // Validate XDR is not empty
    if req.xdr.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "invalid_xdr".to_string(),
            message: "Transaction XDR cannot be empty".to_string(),
        });
    }

    // Submit transaction to Stellar network
    match stellar::submit_transaction(&req.xdr).await {
        Ok(result) => HttpResponse::Ok().json(SendTransactionResponse {
            hash: result.hash,
            ledger: result.ledger,
            success: true,
            message: "Transaction submitted successfully".to_string(),
        }),
        Err(e) => {
            log::error!("Transaction submission failed: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "submission_failed".to_string(),
                message: format!("Transaction submission failed: {}", e),
            })
        }
    }
}

/**
 * GET /health
 * Health check endpoint
 */
pub async fn health_check() -> impl Responder {
    #[derive(Serialize)]
    struct HealthResponse {
        status: String,
        service: String,
        network: String,
    }

    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        service: "stellar-dapp-backend".to_string(),
        network: std::env::var("STELLAR_NETWORK")
            .unwrap_or_else(|_| "TESTNET".to_string()),
    })
}