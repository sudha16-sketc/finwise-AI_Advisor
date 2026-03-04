use soroban_sdk::{Address, Env};
use crate::types::{DataKey, UserData, CommitmentData};
use crate::errors::ContractError;

pub fn get_user(env: &Env, user: &Address) -> UserData {
    env.storage()
        .persistent()
        .get(&DataKey::User(user.clone()))
        .unwrap_or_default()
}

pub fn set_user(env: &Env, user: &Address, data: &UserData) {
    env.storage()
        .persistent()
        .set(&DataKey::User(user.clone()), data);
}

pub fn get_commitment(env: &Env, user: &Address) -> Option<CommitmentData> {
    env.storage()
        .persistent()
        .get(&DataKey::Commitment(user.clone()))
}

pub fn set_commitment(env: &Env, user: &Address, data: &CommitmentData) {
    env.storage()
        .persistent()
        .set(&DataKey::Commitment(user.clone()), data);
}

pub fn get_token(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::Token)
        .ok_or(ContractError::NotInitialized)
}

pub fn set_token(env: &Env, token: &Address) {
    env.storage()
        .instance()
        .set(&DataKey::Token, token);
}