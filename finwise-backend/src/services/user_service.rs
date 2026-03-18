// src/services/user_service.rs
use crate::db::Database;
use crate::models::user::User;
use crate::models::transactions::Transaction;
use mongodb::{bson::doc, Collection};
use chrono::Utc;

/// Track or update user activity (wallet-based)
pub async fn track_user(
    db: &Database,
    wallet_address: &str,
) -> Result<User, String> {
    let users: Collection<User> = db.collection("users");
    let transactions: Collection<Transaction> = db.collection("transactions");

    let filter = doc! { "wallet_address": wallet_address };

    // Serialize chrono datetime to BSON for use inside raw doc! macros
    let now_bson = mongodb::bson::to_bson(&Utc::now())
        .map_err(|e| format!("DateTime serialize error: {}", e))?;

    let update = doc! {
        "$setOnInsert": {
            "username": format!("wallet_{}", &wallet_address[..8.min(wallet_address.len())]),
            "email": format!("{}@stellar.finwise", &wallet_address[..20.min(wallet_address.len())]),
            "password": "",
            "wallet_address": wallet_address,
            "created_at": &now_bson,
            "total_actions": 0i64
        },
        "$set": {
            "last_active": &now_bson
        },
        "$inc": {
            "total_actions": 1i64
        }
    };

    let updated_user = users
        .find_one_and_update(filter, update)
        .upsert(true)
        .return_document(mongodb::options::ReturnDocument::After)
        .await
        .map_err(|e| format!("DB error: {}", e))?;

    if let Some(user) = updated_user {
        // Log a "connect" transaction
        let tx = Transaction {
            id: None,
            wallet_address: wallet_address.to_string(),
            tx_type: "connect".to_string(),
            amount: None,
            created_at: Utc::now(), // ✅ chrono::DateTime<Utc>
        };

        transactions
            .insert_one(tx)
            .await
            .map_err(|e| format!("Tx insert error: {}", e))?;

        Ok(user)
    } else {
        Err("Failed to track user".to_string())
    }
}

/// Log any transaction (deposit / withdraw / connect)
pub async fn log_transaction(
    db: &Database,
    wallet_address: &str,
    tx_type: &str,
    amount: Option<f64>,
) -> Result<(), String> {
    let transactions: Collection<Transaction> = db.collection("transactions");

    let tx = Transaction {
        id: None,
        wallet_address: wallet_address.to_string(),
        tx_type: tx_type.to_string(),
        amount,
        created_at: Utc::now(), // ✅ chrono::DateTime<Utc>
    };

    transactions
        .insert_one(tx)
        .await
        .map_err(|e| format!("Tx insert error: {}", e))?;

    // Update user activity
    track_user(db, wallet_address).await?;

    Ok(())
}