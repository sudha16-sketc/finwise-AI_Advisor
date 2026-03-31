// src/routes/routes.rs
//
// Protected routes require a valid JWT via the AuthUser extractor.
// Returns 401 automatically if the token is missing or invalid.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::stellar;
use crate::utils::auth_extractor::AuthUser;

#[derive(Serialize)]
struct BalanceResponse {
    balance: String,
    address: String,
}

#[derive(Serialize)]
struct TransactionsResponse {
    transactions: Vec<stellar::Transaction>,
    count: usize,
}

#[derive(Deserialize)]
pub struct SendTransactionRequest {
    xdr: String,
}

#[derive(Serialize)]
struct SendTransactionResponse {
    hash: String,
    ledger: i64,
    success: bool,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

/// GET /api/balance/{address}
/// Public — balance checks do not require auth.
pub async fn get_balance(address: web::Path<String>) -> impl Responder {
    let address = address.into_inner();
    log::info!("GET /api/balance/{}", address);

    if !stellar::is_valid_address(&address) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "invalid_address".to_string(),
            message: "Invalid Stellar address format".to_string(),
        });
    }

    match stellar::fetch_account_balance(&address).await {
        Ok(balance) => HttpResponse::Ok().json(BalanceResponse { balance, address }),
            Err(e) => {
                let err_msg = e.to_string();
                log::error!("Failed to fetch balance for {}: {}", address, err_msg);
                if err_msg.contains("Account not found") || err_msg.contains("404") {
                    HttpResponse::NotFound().json(ErrorResponse {
                        error: "account_not_found".to_string(),
                        message: "Stellar account does not exist. Please fund your wallet to activate it.".to_string(),
                    })
                } else {
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "balance_fetch_failed".to_string(),
                        message: err_msg,
                    })
                }
        }
    }
}

/// GET /api/transactions/{address}
/// Protected — requires valid JWT.
pub async fn get_transactions(
    _auth: AuthUser, // 401 returned automatically if missing/invalid
    address: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let address = address.into_inner();
    let limit = query.get("limit").and_then(|l| l.parse::<i32>().ok());

    log::info!("GET /api/transactions/{}", address);

    if !stellar::is_valid_address(&address) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "invalid_address".to_string(),
            message: "Invalid Stellar address format".to_string(),
        });
    }

    match stellar::fetch_transaction_history(&address, limit).await {
        Ok(transactions) => {
            let count = transactions.len();
            HttpResponse::Ok().json(TransactionsResponse { transactions, count })
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

/// POST /api/send
/// Protected — requires valid JWT.
pub async fn send_transaction(
    _auth: AuthUser,
    req: web::Json<SendTransactionRequest>,
) -> impl Responder {
    log::info!("POST /api/send");

    if req.xdr.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "invalid_xdr".to_string(),
            message: "Transaction XDR cannot be empty".to_string(),
        });
    }

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