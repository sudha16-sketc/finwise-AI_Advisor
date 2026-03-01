use soroban_sdk::{Address, Env, symbol_short};

/// Emitted when a user successfully makes a deposit
pub fn emit_deposit(
    env: &Env,
    user: &Address,
    amount: i128,
    new_total: i128,
    streak: u32,
    reward_points: u32,
    timestamp: u64,
) {
    env.events().publish(
        (symbol_short!("deposit"), user.clone()),
        (amount, new_total, streak, reward_points, timestamp),
    );
}

/// Emitted when a user commits to a savings goal
pub fn emit_goal_committed(
    env: &Env,
    user: &Address,
    goal_amount: i128,
    duration_days: u32,
    start_time: u64,
) {
    env.events().publish(
        (symbol_short!("goal_set"), user.clone()),
        (goal_amount, duration_days, start_time),
    );
}

/// Emitted when a user completes their savings goal
pub fn emit_goal_completed(env: &Env, user: &Address, goal_amount: i128, timestamp: u64) {
    env.events().publish(
        (symbol_short!("goal_done"), user.clone()),
        (goal_amount, timestamp),
    );
}