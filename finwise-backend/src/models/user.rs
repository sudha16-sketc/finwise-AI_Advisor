// src/models/user.rs
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

fn default_total_actions() -> u64 {
    0
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub username: String,
    pub email: String,
    pub password: String,

    pub wallet_address: Option<String>,

    // ✅ Stored as BSON Date — enables $gte/$lte date range queries
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime_opt"
    )]
    pub last_active: Option<DateTime<Utc>>,

    #[serde(default = "default_total_actions")]
    pub total_actions: u64,

    // ✅ Stored as BSON Date
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
}