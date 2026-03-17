use crate::db::Database;
use crate::models::user::User;
use crate::models::transactions::Transaction;
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, options::FindOneAndUpdateOptions};
use actix_web::HttpResponse;

pub async fn track_user(db: &Database, wallet_address: &str) -> Result<User, String> {
    let users = db.collection::<User>("users");
    let transactions = db.collection::<Transaction>("transactions");

    // Upsert user: create if new, update last_active + inc actions
    let filter = doc! { "wallet_address": wallet_address };
    let update = doc! {
        "$setOnInsert": {
            "username": &format!("wallet_{}", &wallet_address[0..8]),
            "email": format!("{}@stellar.finwise", &wallet_address[0..20]),
            "password": "", // dummy for wallet users
            "wallet_address": wallet_address,
            "created_at": Utc::now()
        },
        "$set": { "last_active": Utc::now() },
        "$inc": { "total_actions": 1i64 }
    };

    let options = FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .upsert(true)
        .build();

    let updated_user = users
        .find_one_and_update(filter, update, options)
        .await
        .map_err(|e| format!("DB error: {}", e))?;

    if let Some(user) = updated_user {
        // Log transaction
        let tx = Transaction {
            id: None,
            wallet_address: wallet_address.to_string(),
            tx_type: "connect".to_string(),
            amount: None,
            created_at: Utc::now(),
        };
        let _ = transactions.insert_one(tx).await.map_err(|e| format!("Tx insert error: {}", e));

        Ok(user)
    } else {
        Err("Failed to track user".to_string())
    }
}

pub async fn log_transaction(db: &Database, wallet_address: &str, tx_type: &str, amount: Option<f64>) -> Result<(), String> {
    let transactions = db.collection::<Transaction>("transactions");

    let tx = Transaction {
        id: None,
        wallet_address: wallet_address.to_string(),
        tx_type: tx_type.to_string(),
        amount,
        created_at: Utc::now(),
    };

    transactions.insert_one(tx).await
        .map_err(|e| format!("Tx insert error: {}", e))?;

    // Update user activity
    track_user(db, wallet_address).await?;

    Ok(())
}

