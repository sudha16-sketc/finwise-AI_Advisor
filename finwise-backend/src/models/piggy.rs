use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

/// Incoming deposit request
#[derive(Debug, Deserialize)]
pub struct DepositRequest {
    pub user_id: String,
    pub amount: f64,
}

/// MongoDB document for piggy bank state
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PiggyBankDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub total_saved: f64,
    pub current_streak: u32,
    pub longest_streak: u32,
    /// Unix timestamp (seconds) of the last deposit
    pub last_deposit_ts: Option<i64>,
    pub reward_points: u32,
}

/// Stats response to the client
#[derive(Debug, Serialize)]
pub struct PiggyStatsResponse {
    pub user_id: String,
    pub total_saved: f64,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub reward_points: u32,
}

/// Deposit response
#[derive(Debug, Serialize)]
pub struct DepositResponse {
    pub success: bool,
    pub message: String,
    pub total_saved: f64,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub reward_points: u32,
}