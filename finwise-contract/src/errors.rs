use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// Deposit amount must be greater than zero
    InvalidAmount = 1,
    /// User already deposited within the last 24 hours
    AlreadyDepositedToday = 2,
    /// Goal amount must be greater than zero
    InvalidGoalAmount = 3,
    /// Duration must be at least 1 day
    InvalidDuration = 4,
    /// Goal commitment not found for user
    GoalNotFound = 5,
    /// Goal is already completed
    GoalAlreadyCompleted = 6,
    /// Arithmetic overflow
    Overflow = 7,
}