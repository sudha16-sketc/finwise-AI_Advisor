#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn create_token(env: &Env, admin: &Address) -> (Address, StellarAssetClient, TokenClient) {
    let token_id = env.register_stellar_asset_contract_v2(admin.clone());
    let token_address = token_id.address();
    let admin_client = StellarAssetClient::new(env, &token_address);
    let user_client = TokenClient::new(env, &token_address);
    (token_address, admin_client, user_client)
}

fn setup() -> (Env, Address, Address, TokenClient, FinWiseContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let (token_address, token_admin_client, token_client) = create_token(&env, &token_admin);

    let contract_id = env.register_contract(None, FinWiseContract);
    let client = FinWiseContractClient::new(&env, &contract_id);

    client.initialize(&token_address).unwrap();

    let user = Address::generate(&env);
    token_admin_client.mint(&user, &10_000);

    (env, user, token_address, token_client, client)
}

// Day 0 starts at timestamp 0. Using a concrete start makes day boundaries predictable:
//   day 1 → ts >= 86_400
//   day 2 → ts >= 172_800
//   day N → ts >= N * 86_400
const DAY: u64 = 86_400;

#[test]
fn test_first_deposit_moves_tokens() {
    let (env, user, _token_addr, token_client, client) = setup();
    env.ledger().set_timestamp(DAY); // day 1

    let contract_addr = client.address.clone();
    let before = token_client.balance(&user);

    let stats = client.deposit(&user, &500).unwrap();

    assert_eq!(stats.total_saved, 500);
    assert_eq!(stats.current_streak, 1);
    assert_eq!(stats.longest_streak, 1);
    assert_eq!(stats.reward_points, 10);

    assert_eq!(token_client.balance(&user), before - 500);
    assert_eq!(token_client.balance(&contract_addr), 500);
}

#[test]
fn test_withdraw_returns_tokens() {
    let (env, user, _, token_client, client) = setup();
    env.ledger().set_timestamp(DAY);

    let contract_addr = client.address.clone();
    client.deposit(&user, &1_000).unwrap();
    assert_eq!(token_client.balance(&contract_addr), 1_000);

    client.withdraw(&user, &500).unwrap();
    assert_eq!(token_client.balance(&user), 9_500);
    assert_eq!(token_client.balance(&contract_addr), 500);
}

#[test]
fn test_withdraw_exceeding_balance_fails() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(DAY);
    client.deposit(&user, &100).unwrap();

    let err = client.try_withdraw(&user, &999).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InsufficientBalance);
}

#[test]
fn test_duplicate_deposit_same_day_fails() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(DAY); // day 1, 00:00

    client.deposit(&user, &200).unwrap();

    // Still day 1, even 23h 59m later
    env.ledger().set_timestamp(DAY + DAY - 1);
    let err = client.try_deposit(&user, &200).unwrap_err().unwrap();
    assert_eq!(err, ContractError::AlreadyDepositedToday);
}

#[test]
fn test_streak_increments_next_day() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(DAY);     // day 1
    client.deposit(&user, &100).unwrap();

    env.ledger().set_timestamp(DAY * 2); // day 2
    let stats = client.deposit(&user, &100).unwrap();

    assert_eq!(stats.current_streak, 2);
    assert_eq!(stats.longest_streak, 2);
    assert_eq!(stats.total_saved, 200);
}

#[test]
fn test_streak_resets_after_miss() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(DAY);     // day 1
    client.deposit(&user, &100).unwrap();

    // Skip day 2, deposit on day 3
    env.ledger().set_timestamp(DAY * 3); // day 3
    let stats = client.deposit(&user, &100).unwrap();

    assert_eq!(stats.current_streak, 1);  // reset
    assert_eq!(stats.longest_streak, 1);
    assert_eq!(stats.total_saved, 200);
}

#[test]
fn test_three_day_streak() {
    // Regression test for the original bug reported in the screenshot
    let (env, user, _, _, client) = setup();

    env.ledger().set_timestamp(DAY);
    client.deposit(&user, &100).unwrap();

    env.ledger().set_timestamp(DAY * 2);
    client.deposit(&user, &100).unwrap();

    env.ledger().set_timestamp(DAY * 3);
    let stats = client.deposit(&user, &100).unwrap();

    assert_eq!(stats.current_streak, 3);
    assert_eq!(stats.longest_streak, 3);
}

#[test]
fn test_seven_day_streak_bonus() {
    let (env, user, _, _, client) = setup();

    for i in 1..=7u64 {
        env.ledger().set_timestamp(DAY * i);
        client.deposit(&user, &100).unwrap();
    }

    let stats = client.get_user_stats(&user);
    assert_eq!(stats.current_streak, 7);
    // 6 × 10 pts + 1 × (10 + 50) bonus = 60 + 60 = 120
    assert_eq!(stats.reward_points, 120);
}

#[test]
fn test_invalid_amount() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(DAY);

    let err = client.try_deposit(&user, &0).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InvalidAmount);

    let err = client.try_deposit(&user, &-100).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InvalidAmount);
}

#[test]
fn test_commit_and_complete_goal() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(DAY);

    client.commit_goal(&user, &300, &7).unwrap();

    let goal = client.get_goal(&user).unwrap();
    assert_eq!(goal.goal_amount, 300);
    assert!(!goal.completed);

    client.deposit(&user, &150).unwrap();

    env.ledger().set_timestamp(DAY * 2);
    client.deposit(&user, &150).unwrap();

    let goal = client.get_goal(&user).unwrap();
    assert!(goal.completed);
}

#[test]
fn test_invalid_goal_params() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(DAY);

    let err = client.try_commit_goal(&user, &0, &7).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InvalidGoalAmount);

    let err = client.try_commit_goal(&user, &1000, &0).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InvalidDuration);
}

#[test]
fn test_get_stats_defaults_for_new_user() {
    let (env, user, _, _, client) = setup();
    let _ = env;
    let stats = client.get_user_stats(&user);
    assert_eq!(stats.total_saved, 0);
    assert_eq!(stats.current_streak, 0);
    assert_eq!(stats.reward_points, 0);
}

#[test]
fn test_double_initialize_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (token_address, _, _) = create_token(&env, &admin);

    let contract_id = env.register_contract(None, FinWiseContract);
    let client = FinWiseContractClient::new(&env, &contract_id);

    client.initialize(&token_address).unwrap();

    let err = client.try_initialize(&token_address).unwrap_err().unwrap();
    assert_eq!(err, ContractError::AlreadyInitialized);
}