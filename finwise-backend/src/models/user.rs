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

    // last_active is written via BsonDateTime::now() in doc! macros directly
    // so we don't need special serde — just keep it optional for deserialization
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_active: Option<DateTime<Utc>>,

    #[serde(default = "default_total_actions")]
    pub total_actions: u64,

    pub created_at: DateTime<Utc>,
}