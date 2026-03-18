// src/services/metrics_service.rs
use crate::db::Database;
use crate::models::user::User;
use mongodb::bson::{doc, DateTime as BsonDateTime};
use futures_util::TryStreamExt;

#[derive(serde::Serialize)]
pub struct Metrics {
    pub total_users: u64,
    pub active_users_24h: u64,
    pub active_users_7d: u64,
    pub total_transactions: u64,
    pub total_connects: u64,
    pub total_analyses: u64,
    pub avg_actions_per_user: f64,
}

pub async fn get_metrics(db: &Database) -> Result<Metrics, String> {
    let users        = db.collection::<User>("users");
    let transactions = db.collection::<mongodb::bson::Document>("transactions");
    let analyses     = db.collection::<mongodb::bson::Document>("analyses");

    // ✅ BsonDateTime for $gte queries — matches what's stored after the fix
    let now      = BsonDateTime::now();
    let day_ago  = BsonDateTime::from_millis(now.timestamp_millis() - 24 * 60 * 60 * 1000);
    let week_ago = BsonDateTime::from_millis(now.timestamp_millis() - 7 * 24 * 60 * 60 * 1000);

    let total_users = users
        .count_documents(doc! {})
        .await
        .map_err(|e| format!("DB error: {}", e))?;

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

    let total_connects = transactions
        .count_documents(doc! { "tx_type": "connect" })
        .await
        .unwrap_or(0);

    let total_analyses = analyses
        .count_documents(doc! {})
        .await
        .unwrap_or(0);

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
        total_connects,
        total_analyses,
        avg_actions_per_user,
    })
}