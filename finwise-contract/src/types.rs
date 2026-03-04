use soroban_sdk::{contracttype, Address};

/// Storage key enum — each variant represents a distinct key namespace
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Piggy bank stats for a user
    User(Address),
    /// Savings goal commitment for a user
    Commitment(Address),
    /// The SEP-41 token address this contract accepts
    Token,
}

/// On-chain user savings state
#[contracttype]
#[derive(Clone, Debug)]
pub struct UserData {
    pub total_saved: i128,
    pub current_streak: u32,
    pub longest_streak: u32,
    /// Unix timestamp (seconds) of the last deposit
    pub last_deposit_timestamp: u64,
    pub reward_points: u32,
}

impl Default for UserData {
    fn default() -> Self {
        UserData {
            total_saved: 0,
            current_streak: 0,
            longest_streak: 0,
            last_deposit_timestamp: 0,
            reward_points: 0,
        }
    }
}

/// On-chain savings goal commitment
#[contracttype]
#[derive(Clone, Debug)]
pub struct CommitmentData {
    pub goal_amount: i128,
    pub start_time: u64,
    pub duration_days: u32,
    pub amount_deposited: i128,
    pub completed: bool,
}