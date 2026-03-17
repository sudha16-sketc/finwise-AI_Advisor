use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub wallet_address: String,
    pub tx_type: String, // "deposit", "withdraw", "connect"
    pub amount: Option<f64>,
    pub created_at: DateTime<Utc>,
}
