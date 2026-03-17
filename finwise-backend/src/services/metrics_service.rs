use crate::db::Database;
use crate::models::user::User;
use crate::models::transactions::Transaction;

use mongodb::bson::{doc, DateTime};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    pub total_users: u64,
    pub active_users_24h: u64,
    pub active_users_7d: u64,
    pub total_transactions: u64,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
}

pub async fn get_metrics(db: &Database) -> Result<Metrics, String> {
    let users = db.collection::<User>("users");
    let transactions = db.collection::<Transaction>("transactions");

    // ✅ MongoDB-native time handling
    let now = DateTime::now();

    let day_ago = DateTime::from_millis(
        now.timestamp_millis() - 24 * 60 * 60 * 1000
    );

    let week_ago = DateTime::from_millis(
        now.timestamp_millis() - 7 * 24 * 60 * 60 * 1000
    );

    // ✅ Counts (MongoDB 3.5 API — NO None param)
    let total_users = users
        .count_documents(doc! {})
        .await
        .unwrap_or(0);

    let active_users_24h = users
        .count_documents(doc! { "last_active": { "$gte": day_ago } })
        .await
        .unwrap_or(0);

    let active_users_7d = users
        .count_documents(doc! { "last_active": { "$gte": week_ago } })
        .await
        .unwrap_or(0);

    let total_transactions = transactions
        .count_documents(doc! {})
        .await
        .unwrap_or(0);

    let total_deposits = transactions
        .count_documents(doc! { "tx_type": "deposit" })
        .await
        .unwrap_or(0);

    let total_withdrawals = transactions
        .count_documents(doc! { "tx_type": "withdraw" })
        .await
        .unwrap_or(0);

    Ok(Metrics {
        total_users,
        active_users_24h,
        active_users_7d,
        total_transactions,
        total_deposits,
        total_withdrawals,
    })
}