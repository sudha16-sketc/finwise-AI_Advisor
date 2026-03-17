use serde::Serialize;

#[derive(Serialize)]
pub struct Metrics {
    pub total_users: i64,
    pub active_users_24h: i64,
    pub active_users_7d: i64,
    pub total_transactions: i64,
    pub total_deposits: i64,
    pub total_withdrawals: i64,
}