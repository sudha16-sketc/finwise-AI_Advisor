// src/models/user.rs
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime as BsonDateTime;

fn default_total_actions() -> i64 {
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

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_active: Option<BsonDateTime>,

    #[serde(default = "default_total_actions")]
    pub total_actions: i64,

    pub created_at: BsonDateTime,
}