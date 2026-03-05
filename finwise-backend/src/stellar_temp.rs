use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::env;

/**
 * Stellar API service
 * Handles all interactions with the Stellar Horizon API
 */

// Stellar Horizon API base URL
fn get_horizon_url() -> String {
    env::var("HORIZON_URL")
        .unwrap_or_else(|_| "https://horizon-testnet.stellar.org".to_string())
}

/**
 * Account balance response structure
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub balance: String,
    pub asset_type: String,
}

/**
 * Account information from Horizon API
 */
#[derive(Debug, Deserialize)]
struct AccountResponse {
    balances: Vec<Balance>,
}

/**
 * Transaction record structure
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub hash: String,
    pub created_at: String,
    pub source_account: String,
    pub fee_charged: String,
    pub operation_count: i32,
    pub successful: bool,
}

/**
 * Transaction list response from Horizon API
 */
#[derive(Debug, Deserialize)]
struct TransactionsResponse {
    #[serde(rename = "_embedded")]
    embedded: TransactionsEmbedded,
}

#[derive(Debug, Deserialize)]
struct TransactionsEmbedded {
    records: Vec<Transaction>,
}

/**
 * Transaction submission response
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResult {
    pub hash: String,
    pub ledger: i64,
    pub envelope_xdr: Option<String>,
    pub result_xdr: Option<String>,
    pub result_meta_xdr: Option<String>,
}


/**
 * Fetch account balance from Stellar network
 * 
 * # Arguments
 * * `address` - The Stellar public key (G...)
 * 
 * # Returns
 * * XLM balance as a string
 */
pub async fn fetch_account_balance(address: &str) -> Result<String> {
    let url = format!("{}/accounts/{}", get_horizon_url(), address);
    
    log::info!("Fetching balance for address: {}", address);

    // Make HTTP request to Horizon API
    let response = reqwest::get(&url)
        .await
        .map_err(|e| anyhow!("Failed to fetch account: {}", e))?;

    // Check if account exists
    if !response.status().is_success() {
        return Err(anyhow!(
            "Account not found or error fetching balance: {}",
            response.status()
        ));
    }

    // Parse response
    let account: AccountResponse = response
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse account response: {}", e))?;

    // Find native XLM balance
    let xlm_balance = account
        .balances
        .iter()
        .find(|b| b.asset_type == "native")
        .map(|b| b.balance.clone())
        .unwrap_or_else(|| "0".to_string());

    log::info!("Balance for {}: {} XLM", address, xlm_balance);
    Ok(xlm_balance)
}

/**
 * Fetch transaction history for an account
 * 
 * # Arguments
 * * `address` - The Stellar public key (G...)
 * * `limit` - Maximum number of transactions to fetch (default: 10)
 * 
 * # Returns
 * * Vector of transactions
 */
pub async fn fetch_transaction_history(
    address: &str,
    limit: Option<i32>,
) -> Result<Vec<Transaction>> {
    let limit = limit.unwrap_or(10);
    let url = format!(
        "{}/accounts/{}/transactions?order=desc&limit={}",
        get_horizon_url(),
        address,
        limit
    );

    log::info!(
        "Fetching transaction history for address: {} (limit: {})",
        address,
        limit
    );

    // Make HTTP request
    let response = reqwest::get(&url)
        .await
        .map_err(|e| anyhow!("Failed to fetch transactions: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to fetch transactions: {}",
            response.status()
        ));
    }

    // Parse response
    let tx_response: TransactionsResponse = response
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse transactions response: {}", e))?;

    log::info!(
        "Fetched {} transactions for {}",
        tx_response.embedded.records.len(),
        address
    );

    Ok(tx_response.embedded.records)
}

/**
 * Submit a signed transaction to the Stellar network
 * 
 * # Arguments
 * * `transaction_xdr` - Signed transaction in XDR format
 * 
 * # Returns
 * * Transaction result with hash and ledger information
 */
pub async fn submit_transaction(transaction_xdr: &str) -> Result<TransactionResult> {
    let url = format!("{}/transactions", get_horizon_url());

    log::info!("Submitting transaction to Stellar network");

    // Create HTTP client
    let client = reqwest::Client::new();

    // Submit transaction
    let response = client
        .post(&url)
        .form(&[("tx", transaction_xdr)])
        .send()
        .await
        .map_err(|e| anyhow!("Failed to submit transaction: {}", e))?;

    // Check response status
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();

        println!("\n=== HORIZON ERROR START ===");
        println!("{}", error_text);
        println!("=== HORIZON ERROR END ===\n");

        return Err(anyhow!(error_text));
    }


    // Parse response
    let result: TransactionResult = response
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse transaction result: {}", e))?;

    log::info!(
        "Transaction submitted successfully. Hash: {}, Ledger: {}",
        result.hash,
        result.ledger
    );

    Ok(result)
}

/**
 * Validate Stellar address format
 * 
 * # Arguments
 * * `address` - The address to validate
 * 
 * # Returns
 * * true if valid, false otherwise
 */
pub fn is_valid_address(address: &str) -> bool {
    // Basic validation: should start with 'G' and be 56 characters
    address.starts_with('G') && address.len() == 56
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_validation() {
        // Valid testnet address format
        assert!(is_valid_address(
            "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H"
        ));
        
        // Invalid addresses
        assert!(!is_valid_address("invalid"));
        assert!(!is_valid_address(""));
    }
}