use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyDepositedToday = 1,
    InvalidAmount        = 2,
    InvalidGoalAmount    = 3,
    InvalidDuration      = 4,
    AlreadyInitialized   = 5,
    NotInitialized       = 6,
    InsufficientBalance  = 7,
}