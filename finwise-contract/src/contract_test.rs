#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env};

fn setup() -> (Env, Address, FinWiseContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, FinWiseContract);
    let client = FinWiseContractClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    (env, user, client)
}

#[test]
fn test_first_deposit() {
    let (env, user, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    let stats = client.deposit(&user, &500).unwrap();
    assert_eq!(stats.total_saved, 500);
    assert_eq!(stats.current_streak, 1);
    assert_eq!(stats.longest_streak, 1);
    assert_eq!(stats.reward_points, 10); // Base +10
}

#[test]
fn test_duplicate_deposit_same_day_fails() {
    let (env, user, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    client.deposit(&user, &200).unwrap();

    // Same day — should fail
    let err = client.try_deposit(&user, &200).unwrap_err().unwrap();
    assert_eq!(err, ContractError::AlreadyDepositedToday);
}

#[test]
fn test_streak_increments_next_day() {
    let (env, user, client) = setup();
    env.ledger().set_timestamp(1_000_000);
    client.deposit(&user, &100).unwrap();

    // Advance 25 hours (still within grace window)
    env.ledger().set_timestamp(1_000_000 + 86_400 + 1);
    let stats = client.deposit(&user, &100).unwrap();

    assert_eq!(stats.current_streak, 2);
    assert_eq!(stats.total_saved, 200);
}

#[test]
fn test_streak_resets_after_miss() {
    let (env, user, client) = setup();
    env.ledger().set_timestamp(1_000_000);
    client.deposit(&user, &100).unwrap();

    // Skip 2 days — streak should reset
    env.ledger().set_timestamp(1_000_000 + 86_400 * 2 + 1);
    let stats = client.deposit(&user, &100).unwrap();

    assert_eq!(stats.current_streak, 1); // Reset
    assert_eq!(stats.longest_streak, 1);
    assert_eq!(stats.total_saved, 200); // Total still accumulates
}

#[test]
fn test_seven_day_streak_bonus() {
    let (env, user, client) = setup();
    let mut ts = 1_000_000_u64;

    for _ in 0..7 {
        env.ledger().set_timestamp(ts);
        client.deposit(&user, &100).unwrap();
        ts += 86_401; // Next day
    }

    let stats = client.get_user_stats(&user);
    assert_eq!(stats.current_streak, 7);
    // 6 deposits × 10 pts + 1 deposit with 7-day bonus (10 + 50) = 60 + 60 = 120
    assert_eq!(stats.reward_points, 120);
}

#[test]
fn test_invalid_amount() {
    let (env, user, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    let err = client.try_deposit(&user, &0).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InvalidAmount);

    let err = client.try_deposit(&user, &-100).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InvalidAmount);
}

#[test]
fn test_commit_and_complete_goal() {
    let (env, user, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    // Commit to saving 300 in 7 days
    client.commit_goal(&user, &300, &7).unwrap();

    let goal = client.get_goal(&user).unwrap();
    assert_eq!(goal.goal_amount, 300);
    assert!(!goal.completed);

    // Deposit 150 (Day 1)
    client.deposit(&user, &150).unwrap();
    // Deposit 150 (Day 2)
    env.ledger().set_timestamp(1_000_000 + 86_401);
    client.deposit(&user, &150).unwrap();

    let goal = client.get_goal(&user).unwrap();
    assert!(goal.completed);
}

#[test]
fn test_invalid_goal_params() {
    let (env, user, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    let err = client.try_commit_goal(&user, &0, &7).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InvalidGoalAmount);

    let err = client.try_commit_goal(&user, &1000, &0).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InvalidDuration);
}

#[test]
fn test_get_stats_defaults_for_new_user() {
    let (env, user, client) = setup();
    let _ = env;
    let stats = client.get_user_stats(&user);
    assert_eq!(stats.total_saved, 0);
    assert_eq!(stats.current_streak, 0);
    assert_eq!(stats.reward_points, 0);
}
