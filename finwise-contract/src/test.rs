#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

/// Deploy a mock SEP-41 token (Stellar Asset Contract) for testing.
/// Returns (token_address, token_admin_client, token_user_client)
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

    // Deploy token
    let token_admin = Address::generate(&env);
    let (token_address, token_admin_client, token_client) = create_token(&env, &token_admin);

    // Deploy FinWise contract
    let contract_id = env.register_contract(None, FinWiseContract);
    let client = FinWiseContractClient::new(&env, &contract_id);

    // Initialize with the token address
    client.initialize(&token_address).unwrap();

    // Create user and mint them 10_000 tokens
    let user = Address::generate(&env);
    token_admin_client.mint(&user, &10_000);

    (env, user, token_address, token_client, client)
}

#[test]
fn test_first_deposit_moves_tokens() {
    let (env, user, _token_addr, token_client, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    let contract_addr = client.address.clone();
    let user_balance_before = token_client.balance(&user);

    let stats = client.deposit(&user, &500).unwrap();

    // Check on-chain stats
    assert_eq!(stats.total_saved, 500);
    assert_eq!(stats.current_streak, 1);
    assert_eq!(stats.longest_streak, 1);
    assert_eq!(stats.reward_points, 10);

    // ✅ Verify real token transfer occurred
    assert_eq!(token_client.balance(&user), user_balance_before - 500);
    assert_eq!(token_client.balance(&contract_addr), 500);
}

#[test]
fn test_withdraw_returns_tokens() {
    let (env, user, _token_addr, token_client, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    let contract_addr = client.address.clone();

    client.deposit(&user, &1_000).unwrap();
    assert_eq!(token_client.balance(&contract_addr), 1_000);

    // Withdraw half
    client.withdraw(&user, &500).unwrap();

    assert_eq!(token_client.balance(&user), 9_500); // 10_000 - 1_000 + 500
    assert_eq!(token_client.balance(&contract_addr), 500);
}

#[test]
fn test_withdraw_exceeding_balance_fails() {
    let (env, user, _token_addr, _token_client, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    client.deposit(&user, &100).unwrap();

    let err = client.try_withdraw(&user, &999).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InsufficientBalance);
}

#[test]
fn test_duplicate_deposit_same_day_fails() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    client.deposit(&user, &200).unwrap();

    let err = client.try_deposit(&user, &200).unwrap_err().unwrap();
    assert_eq!(err, ContractError::AlreadyDepositedToday);
}

#[test]
fn test_streak_increments_next_day() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(1_000_000);
    client.deposit(&user, &100).unwrap();

    env.ledger().set_timestamp(1_000_000 + 86_401);
    let stats = client.deposit(&user, &100).unwrap();

    assert_eq!(stats.current_streak, 2);
    assert_eq!(stats.total_saved, 200);
}

#[test]
fn test_streak_resets_after_miss() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(1_000_000);
    client.deposit(&user, &100).unwrap();

    // Skip 2 days — streak broken
    env.ledger().set_timestamp(1_000_000 + 86_400 * 2 + 1);
    let stats = client.deposit(&user, &100).unwrap();

    assert_eq!(stats.current_streak, 1);
    assert_eq!(stats.longest_streak, 1);
    assert_eq!(stats.total_saved, 200);
}

#[test]
fn test_seven_day_streak_bonus() {
    let (env, user, _, _, client) = setup();
    let mut ts = 1_000_000_u64;

    for _ in 0..7 {
        env.ledger().set_timestamp(ts);
        client.deposit(&user, &100).unwrap();
        ts += 86_401;
    }

    let stats = client.get_user_stats(&user);
    assert_eq!(stats.current_streak, 7);
    // 6 × 10pts + 1 × (10 + 50) = 60 + 60 = 120
    assert_eq!(stats.reward_points, 120);
}

#[test]
fn test_invalid_amount() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    let err = client.try_deposit(&user, &0).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InvalidAmount);

    let err = client.try_deposit(&user, &-100).unwrap_err().unwrap();
    assert_eq!(err, ContractError::InvalidAmount);
}

#[test]
fn test_commit_and_complete_goal() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(1_000_000);

    client.commit_goal(&user, &300, &7).unwrap();

    let goal = client.get_goal(&user).unwrap();
    assert_eq!(goal.goal_amount, 300);
    assert!(!goal.completed);

    client.deposit(&user, &150).unwrap();
    env.ledger().set_timestamp(1_000_000 + 86_401);
    client.deposit(&user, &150).unwrap();

    let goal = client.get_goal(&user).unwrap();
    assert!(goal.completed);
}

#[test]
fn test_invalid_goal_params() {
    let (env, user, _, _, client) = setup();
    env.ledger().set_timestamp(1_000_000);

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