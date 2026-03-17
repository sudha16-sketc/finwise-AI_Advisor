use crate::db::Database;
use crate::models::user::User;
use crate::models::transactions::Transaction;
use mongodb::bson::{doc, DateTime as BsonDateTime};
use chrono::{Duration, Utc};
use serde_json::json;

#[derive(serde::Serialize)]
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

    // Total users
    let total_users = users.count_documents(doc! {}, None).await
        .map_err(|e| format!("Count users error: {}", e))?;

    // Active users 24h/7d (last_active)
    let now = Utc::now();
    let day_ago = BsonDateTime(now - Duration::hours(24));
    let week_ago = BsonDateTime(now - Duration::days(7));

    let active_24h_filter = doc! { "last_active": { "$gte": day_ago } };
    let active_7d_filter = doc! { "last_active": { "$gte": week_ago } };

    let active_24h = users.count_documents(active_24h_filter, None).await
        .map_err(|e| format!("Count active24h error: {}", e))?;
    let active_7d = users.count_documents(active_7d_filter, None).await
        .map_err(|e| format!("Count active7d error: {}", e))?;

    // Tx counts by type
    let total_tx_pipeline = vec![
        doc! { "$group": { "_id": null, "count": { "$sum": 1 } } }
    ];
    let deposits_pipeline = vec![
        doc! { "$match": { "tx_type": "deposit" } },
        doc! { "$group": { "_id": null, "count": { "$sum": 1 } } }
    ];
    let withdrawals_pipeline = vec![
        doc! { "$match": { "tx_type": "withdraw" } },
        doc! { "$group": { "_id": null, "count": { "$sum": 1 } } }
    ];

    let total_tx_result: Option<serde_json::Value> = transactions.aggregate(total_tx_pipeline, None).await
        .map_err(|e| format!("Agg total_tx error: {}", e))?
        .try_next()
        .await
        .map_err(|e| format!("Agg next error: {}", e))?
        .map(|doc| doc.get_i32("count").unwrap_or(0) as u64);

    let deposits_result: Option<u64> = transactions.aggregate(deposits_pipeline, None).await
        .map_err(|e| format!("Agg deposits error: {}", e))?
        .try_next()
        .await
        .map_err(|e| format!("Agg next error: {}", e))?
        .map(|doc| doc.get_i32("count").unwrap_or(0) as u64);

    let withdrawals_result: Option<u64> = transactions.aggregate(withdrawals_pipeline, None).await
        .map_err(|e| format!("Agg withdrawals error: {}", e))?
        .try_next()
        .await
        .map_err(|e| format!("Agg next error: {}", e))?
        .map(|doc| doc.get_i32("count").unwrap_or(0) as u64);

    let metrics = Metrics {
        total_users,
        active_users_24h: active_24h,
        active_users_7d: active_7d,
        total_transactions: total_tx_result.unwrap_or(0),
        total_deposits: deposits_result.unwrap_or(0),
        total_withdrawals: withdrawals_result.unwrap_or(0),
    };

    Ok(metrics)
}

