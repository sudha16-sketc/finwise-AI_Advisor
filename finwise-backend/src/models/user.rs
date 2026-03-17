use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub username: String,
    pub email: String,
    pub password: String,

    pub wallet_address: Option<String>,
    pub last_active: Option<DateTime<Utc>>,
    pub total_actions: u64,

    pub created_at: DateTime<Utc>,
}