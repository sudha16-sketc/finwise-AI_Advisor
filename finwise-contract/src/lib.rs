#![no_std]

mod errors;
mod events;
mod storage;
mod utils;

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

use errors::ContractError;
use storage::{CommitmentData, DataKey, UserData};
use utils::{calculate_reward_points, is_new_day, streak_broken};

/// Return type for get_user_stats
#[contracttype]
#[derive(Clone, Debug)]
pub struct UserStats {
    pub total_saved: i128,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub reward_points: u32,
}

#[contract]
pub struct FinWiseContract;

#[contractimpl]
impl FinWiseContract {
    // =========================================================================
    // DEPOSIT
    // =========================================================================

    /// Records a daily savings deposit for the user on-chain.
    ///
    /// - Requires user authorization (prevents unauthorized deposits)
    /// - Enforces one deposit per 24 hours
    /// - Tracks total saved, streak, and reward points
    /// - Resets streak if more than 25 hours have passed since last deposit
    pub fn deposit(env: Env, user: Address, amount: i128) -> Result<UserStats, ContractError> {
        // Require the user to sign this transaction
        user.require_auth();

        // Validate amount
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        let now: u64 = env.ledger().timestamp();

        // Load or initialize user state
        let mut data: UserData = env
            .storage()
            .persistent()
            .get(&DataKey::User(user.clone()))
            .unwrap_or_default();

        // Enforce one deposit per 24 hours
        if data.last_deposit_timestamp > 0 && !is_new_day(data.last_deposit_timestamp, now) {
            return Err(ContractError::AlreadyDepositedToday);
        }

        // Reset or extend streak
        if data.last_deposit_timestamp > 0 && streak_broken(data.last_deposit_timestamp, now) {
            data.current_streak = 0;
        }

        // Update stats (saturating arithmetic prevents overflow)
        data.total_saved = data.total_saved.saturating_add(amount);
        data.current_streak = data.current_streak.saturating_add(1);
        data.last_deposit_timestamp = now;

        if data.current_streak > data.longest_streak {
            data.longest_streak = data.current_streak;
        }

        // Add reward points
        let earned_points = calculate_reward_points(data.current_streak);
        data.reward_points = data.reward_points.saturating_add(earned_points);

        // Emit deposit event (off-chain backend can listen)
        events::emit_deposit(
            &env,
            &user,
            amount,
            data.total_saved,
            data.current_streak,
            data.reward_points,
            now,
        );

        // Persist state
        env.storage()
            .persistent()
            .set(&DataKey::User(user.clone()), &data);

        // Also check and update goal progress if user has a commitment
        if let Some(mut commitment) =
            env.storage()
                .persistent()
                .get::<DataKey, CommitmentData>(&DataKey::Commitment(user.clone()))
        {
            if !commitment.completed {
                commitment.amount_deposited =
                    commitment.amount_deposited.saturating_add(amount);

                if commitment.amount_deposited >= commitment.goal_amount {
                    commitment.completed = true;
                    events::emit_goal_completed(&env, &user, commitment.goal_amount, now);
                }

                env.storage()
                    .persistent()
                    .set(&DataKey::Commitment(user.clone()), &commitment);
            }
        }

        Ok(UserStats {
            total_saved: data.total_saved,
            current_streak: data.current_streak,
            longest_streak: data.longest_streak,
            reward_points: data.reward_points,
        })
    }

    // =========================================================================
    // GET USER STATS
    // =========================================================================

    /// Returns on-chain savings stats for the given user.
    pub fn get_user_stats(env: Env, user: Address) -> UserStats {
        let data: UserData = env
            .storage()
            .persistent()
            .get(&DataKey::User(user))
            .unwrap_or_default();

        UserStats {
            total_saved: data.total_saved,
            current_streak: data.current_streak,
            longest_streak: data.longest_streak,
            reward_points: data.reward_points,
        }
    }

    // =========================================================================
    // COMMIT GOAL
    // =========================================================================

    /// Creates an on-chain savings goal commitment for the user.
    ///
    /// - goal_amount: Target amount to save (in smallest token unit)
    /// - duration_days: Number of days to achieve the goal
    pub fn commit_goal(
        env: Env,
        user: Address,
        goal_amount: i128,
        duration_days: u32,
    ) -> Result<(), ContractError> {
        user.require_auth();

        if goal_amount <= 0 {
            return Err(ContractError::InvalidGoalAmount);
        }
        if duration_days == 0 {
            return Err(ContractError::InvalidDuration);
        }

        let now: u64 = env.ledger().timestamp();

        let commitment = CommitmentData {
            goal_amount,
            start_time: now,
            duration_days,
            amount_deposited: 0,
            completed: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Commitment(user.clone()), &commitment);

        events::emit_goal_committed(&env, &user, goal_amount, duration_days, now);

        Ok(())
    }

    // =========================================================================
    // GET GOAL
    // =========================================================================

    /// Returns the user's active savings goal commitment, if any.
    pub fn get_goal(env: Env, user: Address) -> Option<CommitmentData> {
        env.storage()
            .persistent()
            .get(&DataKey::Commitment(user))
    }
}