//  src/models/profile.rs
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

use crate::models::analysis::FinancialAdvice;

/// MongoDB document for user profile / latest advice
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub latest_advice: Option<FinancialAdvice>,
    pub total_analyses: u32,
    pub updated_at: DateTime<Utc>,
    // ↓ Add #[serde(default)] so missing fields deserialize as 0 instead of erroring
    #[serde(default)]
    pub total_saved: f64,
    #[serde(default)]
    pub current_streak: u32,
    #[serde(default)]
    pub longest_streak: u32,
    #[serde(default)]
    pub reward_points: u32,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub user_id: String,
    pub latest_advice: Option<FinancialAdvice>,
    pub total_analyses: u32,
    pub total_saved: f64,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub reward_points: u32,
}