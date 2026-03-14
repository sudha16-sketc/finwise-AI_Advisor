#![no_std]

mod types;
mod storage;
mod events;
mod errors;

use soroban_sdk::{contract, contractimpl, Address, Env, token};
use types::{UserData, CommitmentData, DataKey};
use errors::ContractError;
use storage::{get_user, set_user, get_commitment, set_commitment, get_token, set_token};
use events::{emit_deposit, emit_withdraw, emit_goal_committed, emit_goal_completed};

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Convert a Unix timestamp to a calendar day number (days since epoch).
/// Two timestamps share the same day number iff they fall on the same UTC day.
fn day_number(ts: u64) -> u64 {
    ts / 86_400
}

fn calculate_reward_points(current_streak: u32) -> u32 {
    let mut pts: u32 = 10;
    if current_streak == 7  { pts = pts.saturating_add(50);  }
    if current_streak == 30 { pts = pts.saturating_add(200); }
    pts
}

// ─── Contract ────────────────────────────────────────────────────────────────

#[contract]
pub struct FinWiseContract;

#[contractimpl]
impl FinWiseContract {
    /// Initialize the contract with the SEP-41 token to use for deposits.
    /// Must be called once before any other function.
    pub fn initialize(env: Env, token: Address) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Token) {
            return Err(ContractError::AlreadyInitialized);
        }
        set_token(&env, &token);
        Ok(())
    }

    /// Deposit `amount` tokens into the user's FinWise piggy bank.
    ///
    /// Rules:
    ///  - One deposit per UTC calendar day (not a rolling 24-hour window).
    ///  - Depositing on consecutive days increments the streak.
    ///  - Skipping a day (gap > 1 calendar day) resets the streak to 1.
    pub fn deposit(env: Env, user: Address, amount: i128) -> Result<UserData, ContractError> {
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        user.require_auth();

        let now = env.ledger().timestamp();
        let today = day_number(now);
        let mut data = get_user(&env, &user);

        // ── Streak + duplicate-deposit logic (calendar-day based) ──────────
        if data.last_deposit_timestamp == 0 {
            // Very first deposit ever
            data.current_streak = 1;
        } else {
            let last_day = day_number(data.last_deposit_timestamp);

            if today == last_day {
                // Same UTC day → reject duplicate
                return Err(ContractError::AlreadyDepositedToday);
            } else if today == last_day + 1 {
                // Next consecutive day → extend streak
                data.current_streak = data.current_streak.saturating_add(1);
            } else {
                // Skipped one or more days → reset streak
                data.current_streak = 1;
            }
        }

        if data.current_streak > data.longest_streak {
            data.longest_streak = data.current_streak;
        }

        // ── Reward points ──────────────────────────────────────────────────
        let pts = calculate_reward_points(data.current_streak);
        data.reward_points = data.reward_points.saturating_add(pts);

        // ── Token transfer: user → contract ────────────────────────────────
        let token_address = get_token(&env)?;
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(
            &user,
            &env.current_contract_address(),
            &amount,
        );

        // ── Update state AFTER transfer succeeds ───────────────────────────
        data.total_saved = data.total_saved.saturating_add(amount);
        data.last_deposit_timestamp = now;

        let goal_completed = Self::update_goal_progress(&env, &user, amount);

        set_user(&env, &user, &data);

        emit_deposit(&env, &user, amount, &data);
        if goal_completed {
            emit_goal_completed(&env, &user);
        }

        Ok(data)
    }

    /// Withdraw `amount` tokens back to the user.
    pub fn withdraw(env: Env, user: Address, amount: i128) -> Result<UserData, ContractError> {
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        user.require_auth();

        let mut data = get_user(&env, &user);

        if amount > data.total_saved {
            return Err(ContractError::InsufficientBalance);
        }

        let token_address = get_token(&env)?;
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(
            &env.current_contract_address(),
            &user,
            &amount,
        );

        data.total_saved = data.total_saved.saturating_sub(amount);
        set_user(&env, &user, &data);

        emit_withdraw(&env, &user, amount, &data);

        Ok(data)
    }

    /// Commit to a savings goal.
    pub fn commit_goal(
        env: Env,
        user: Address,
        goal_amount: i128,
        duration_days: u32,
    ) -> Result<(), ContractError> {
        if goal_amount <= 0 {
            return Err(ContractError::InvalidGoalAmount);
        }
        if duration_days == 0 {
            return Err(ContractError::InvalidDuration);
        }

        user.require_auth();

        let commitment = CommitmentData {
            goal_amount,
            start_time: env.ledger().timestamp(),
            duration_days,
            amount_deposited: 0,
            completed: false,
        };

        set_commitment(&env, &user, &commitment);
        emit_goal_committed(&env, &user, goal_amount, duration_days);

        Ok(())
    }

    // ── View functions ────────────────────────────────────────────────────

    pub fn get_user_stats(env: Env, user: Address) -> UserData {
        get_user(&env, &user)
    }

    pub fn get_goal(env: Env, user: Address) -> Option<CommitmentData> {
        get_commitment(&env, &user)
    }

    pub fn get_token_address(env: Env) -> Result<Address, ContractError> {
        get_token(&env)
    }

    // ── Internal helpers ──────────────────────────────────────────────────

    fn update_goal_progress(env: &Env, user: &Address, amount: i128) -> bool {
        let Some(mut goal) = get_commitment(env, user) else {
            return false;
        };

        if goal.completed {
            return false;
        }

        goal.amount_deposited = goal.amount_deposited.saturating_add(amount);

        if goal.amount_deposited >= goal.goal_amount {
            goal.completed = true;
            set_commitment(env, user, &goal);
            return true;
        }

        set_commitment(env, user, &goal);
        false
    }
}