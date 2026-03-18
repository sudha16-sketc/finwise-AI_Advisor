// src/services/metrics_service.rs
use crate::db::Database;
use crate::models::user::User;
use crate::models::transactions::Transaction;
use mongodb::bson::doc;
use chrono::Utc;
use futures_util::TryStreamExt;

#[derive(serde::Serialize)]
pub struct Metrics {
    pub total_users: u64,
    pub active_users_24h: u64,
    pub active_users_7d: u64,
    pub total_transactions: u64,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub total_analyses: u64,
    pub avg_actions_per_user: f64,
}

pub async fn get_metrics(db: &Database) -> Result<Metrics, String> {
    let users        = db.collection::<User>("users");
    let transactions = db.collection::<Transaction>("transactions");
    let analyses     = db.collection::<mongodb::bson::Document>("analyses");

    // ✅ chrono datetimes serialized to BSON — matches how fields are stored
    let now      = Utc::now();
    let day_ago  = now - chrono::Duration::hours(24);
    let week_ago = now - chrono::Duration::days(7);

    let day_ago_bson = mongodb::bson::to_bson(&day_ago)
        .map_err(|e| format!("BSON serialize error: {}", e))?;
    let week_ago_bson = mongodb::bson::to_bson(&week_ago)
        .map_err(|e| format!("BSON serialize error: {}", e))?;

    let total_users = users
        .count_documents(doc! {})
        .await
        .map_err(|e| format!("DB error: {}", e))?;

    let active_users_24h = users
        .count_documents(doc! { "last_active": { "$gte": &day_ago_bson } })
        .await
        .unwrap_or(0);

    let active_users_7d = users
        .count_documents(doc! { "last_active": { "$gte": &week_ago_bson } })
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

    let total_analyses = analyses
        .count_documents(doc! {})
        .await
        .unwrap_or(0);

    // Average actions per user via aggregation
    let avg_actions_per_user = if total_users > 0 {
        let pipeline = vec![
            doc! { "$group": { "_id": null, "total": { "$sum": "$total_actions" } } }
        ];

        let mut cursor = users
            .aggregate(pipeline)
            .await
            .map_err(|e| format!("Aggregation error: {}", e))?;

        let mut total_actions_sum: f64 = 0.0;
        while let Some(doc) = cursor.try_next().await.unwrap_or(None) {
            total_actions_sum = doc
                .get("total")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as f64;
        }
        total_actions_sum / total_users as f64
    } else {
        0.0
    };

    Ok(Metrics {
        total_users,
        active_users_24h,
        active_users_7d,
        total_transactions,
        total_deposits,
        total_withdrawals,
        total_analyses,
        avg_actions_per_user,
    })
}