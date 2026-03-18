// src/services/metrics_service.rs
use crate::db::Database;
use crate::models::user::User;
use mongodb::bson::doc;
use chrono::Utc;
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
    // Use Document for collections we only count — avoids needing typed models
    let transactions = db.collection::<mongodb::bson::Document>("transactions");
    let analyses     = db.collection::<mongodb::bson::Document>("analyses");

    let now      = Utc::now();
    let day_ago  = now - chrono::Duration::hours(24);
    let week_ago = now - chrono::Duration::days(7);

    // Serialize chrono to BSON — matches how last_active is stored
    let day_ago_bson = mongodb::bson::to_bson(&day_ago)
        .map_err(|e| format!("BSON serialize error: {}", e))?;
    let week_ago_bson = mongodb::bson::to_bson(&week_ago)
        .map_err(|e| format!("BSON serialize error: {}", e))?;

    let total_users = users
        .count_documents(doc! {})
        .await
        .map_err(|e| format!("DB error: {}", e))?;

    // ✅ active_24h and active_7d now work because check_auth updates last_active
    let active_users_24h = users
        .count_documents(doc! { "last_active": { "$gte": &day_ago_bson } })
        .await
        .unwrap_or(0);

    let active_users_7d = users
        .count_documents(doc! { "last_active": { "$gte": &week_ago_bson } })
        .await
        .unwrap_or(0);

    // All transactions (connect type logged on every wallet connect / check-auth)
    let total_transactions = transactions
        .count_documents(doc! {})
        .await
        .unwrap_or(0);

    // Connect events specifically
    let total_connects = transactions
        .count_documents(doc! { "tx_type": "connect" })
        .await
        .unwrap_or(0);

    // AI analyses run
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
        total_connects,
        total_analyses,
        avg_actions_per_user,
    })
}