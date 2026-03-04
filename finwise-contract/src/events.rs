/// events.rs — Soroban event emissions
///
/// These events are emitted on-chain and indexed by Horizon / Mercury / Subquery.
/// Frontend apps can subscribe to them for real-time deposit/withdrawal notifications.
///
/// Event schema: (topics...) → data
/// Topics are used for filtering. Data is the payload.

use soroban_sdk::{symbol_short, Address, Env};
use crate::types::UserData;

/// Emitted on every successful deposit.
/// Topic:  ("deposit", user_address)
/// Data:   (amount: i128, streak: u32, reward_points: u32, total_saved: i128)
pub fn emit_deposit(env: &Env, user: &Address, amount: i128, data: &UserData) {
    env.events().publish(
        (symbol_short!("deposit"), user.clone()),
        (amount, data.current_streak, data.reward_points, data.total_saved),
    );
}

/// Emitted on every successful withdrawal.
/// Topic:  ("withdraw", user_address)
/// Data:   (amount: i128, remaining_saved: i128)
pub fn emit_withdraw(env: &Env, user: &Address, amount: i128, data: &UserData) {
    env.events().publish(
        (symbol_short!("withdraw"), user.clone()),
        (amount, data.total_saved),
    );
}

/// Emitted when a user commits to a new savings goal.
/// Topic:  ("goal_set", user_address)
/// Data:   (goal_amount: i128, duration_days: u32)
pub fn emit_goal_committed(env: &Env, user: &Address, goal_amount: i128, duration_days: u32) {
    env.events().publish(
        (symbol_short!("goal_set"), user.clone()),
        (goal_amount, duration_days),
    );
}

/// Emitted when a user's savings goal is fully achieved.
/// Topic:  ("goal_done", user_address)
/// Data:   none (goal address is in the topic)
pub fn emit_goal_completed(env: &Env, user: &Address) {
    env.events().publish(
        (symbol_short!("goal_done"), user.clone()),
        (),
    );
}