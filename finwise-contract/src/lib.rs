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

fn is_new_day(last_ts: u64, now: u64) -> bool {
    const SECONDS_IN_DAY: u64 = 86_400;
    now.saturating_sub(last_ts) >= SECONDS_IN_DAY
}

fn streak_broken(last_ts: u64, now: u64) -> bool {
    const GRACE_PERIOD: u64 = 90_000; // 25 hours
    now.saturating_sub(last_ts) > GRACE_PERIOD
}

fn calculate_reward_points(current_streak: u32) -> u32 {
    let mut pts: u32 = 10;
    if current_streak == 7 {
        pts = pts.saturating_add(50);
    }
    if current_streak == 30 {
        pts = pts.saturating_add(200);
    }
    pts
}

#[contract]
pub struct FinWiseContract;

#[contractimpl]
impl FinWiseContract {
    /// Initialize the contract with the SEP-41 token to use for deposits.
    /// Must be called once before any other function.
    /// `token` — address of any SEP-41 compliant token (USDC, XLM SAC, etc.)
    pub fn initialize(env: Env, token: Address) -> Result<(), ContractError> {
        // Prevent re-initialization
        if env.storage().instance().has(&DataKey::Token) {
            return Err(ContractError::AlreadyInitialized);
        }
        set_token(&env, &token);
        Ok(())
    }

    /// Deposit `amount` tokens into the user's FinWise piggy bank.
    ///
    /// This performs a real SEP-41 token transfer from `user` → contract.
    /// Requires the user to have pre-authorized this transfer (via Stellar auth).
    ///
    /// Returns updated UserData on success.
    pub fn deposit(env: Env, user: Address, amount: i128) -> Result<UserData, ContractError> {
        // --- Validation ---
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Require the user to have signed this transaction
        user.require_auth();

        let now = env.ledger().timestamp();
        let mut data = get_user(&env, &user);

        // --- Enforce one deposit per day ---
        if data.last_deposit_timestamp > 0 && !is_new_day(data.last_deposit_timestamp, now) {
            return Err(ContractError::AlreadyDepositedToday);
        }

        // --- Streak logic ---
        if data.last_deposit_timestamp == 0 {
            // First ever deposit
            data.current_streak = 1;
        } else if streak_broken(data.last_deposit_timestamp, now) {
            // Missed a day — reset streak
            data.current_streak = 1;
        } else {
            // Consecutive day — extend streak
            data.current_streak = data.current_streak.saturating_add(1);
        }

        if data.current_streak > data.longest_streak {
            data.longest_streak = data.current_streak;
        }

        // --- Reward points ---
        let pts = calculate_reward_points(data.current_streak);
        data.reward_points = data.reward_points.saturating_add(pts);

        // --- INTER-CONTRACT CALL: Transfer tokens from user → this contract ---
        // This is the real on-chain movement of funds.
        // The user must have signed an auth envelope that includes this transfer.
        let token_address = get_token(&env)?;
        let token_client = token::Client::new(&env, &token_address);

        token_client.transfer(
            &user,                        // from
            &env.current_contract_address(), // to (this contract holds the funds)
            &amount,                      // amount (in token's smallest unit, e.g. stroops for XLM)
        );

        // --- Update state AFTER transfer succeeds ---
        data.total_saved = data.total_saved.saturating_add(amount);
        data.last_deposit_timestamp = now;

        // --- Check goal progress ---
        let goal_completed = Self::update_goal_progress(&env, &user, amount);

        set_user(&env, &user, &data);

        // --- Emit events for indexers / frontend listeners ---
        emit_deposit(&env, &user, amount, &data);
        if goal_completed {
            emit_goal_completed(&env, &user);
        }

        Ok(data)
    }

    /// Withdraw `amount` tokens back to the user.
    ///
    /// This performs a real SEP-41 token transfer from contract → user.
    /// Only the user themselves can withdraw their own funds.
    pub fn withdraw(env: Env, user: Address, amount: i128) -> Result<UserData, ContractError> {
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        user.require_auth();

        let mut data = get_user(&env, &user);

        if amount > data.total_saved {
            return Err(ContractError::InsufficientBalance);
        }

        // --- INTER-CONTRACT CALL: Transfer tokens from this contract → user ---
        let token_address = get_token(&env)?;
        let token_client = token::Client::new(&env, &token_address);

        token_client.transfer(
            &env.current_contract_address(), // from (contract releases funds)
            &user,                           // to
            &amount,
        );

        // --- Update state AFTER transfer succeeds ---
        data.total_saved = data.total_saved.saturating_sub(amount);
        set_user(&env, &user, &data);

        emit_withdraw(&env, &user, amount, &data);

        Ok(data)
    }

    /// Commit to a savings goal.
    /// `goal_amount` — total tokens to save (in token's smallest unit)
    /// `duration_days` — number of days to achieve it
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

    // --- View functions ---

    pub fn get_user_stats(env: Env, user: Address) -> UserData {
        get_user(&env, &user)
    }

    pub fn get_goal(env: Env, user: Address) -> Option<CommitmentData> {
        get_commitment(&env, &user)
    }

    pub fn get_token_address(env: Env) -> Result<Address, ContractError> {
        get_token(&env)
    }

    // --- Internal helpers ---

    /// Updates the user's commitment progress. Returns true if goal was just completed.
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