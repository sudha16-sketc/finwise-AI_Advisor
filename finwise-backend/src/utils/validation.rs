// src/utils/validation.rs
//
// Manual input validation — no external validator crate required.

use std::fmt;

#[derive(Debug)]
pub struct ValidationError(pub String);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Email must contain exactly one '@' with a non-empty local and domain part
/// that includes at least one '.'.
pub fn validate_email(email: &str) -> Result<(), String> {
    let email = email.trim();
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err("Invalid email format".to_string());
    }
    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || domain.is_empty() || !domain.contains('.') {
        return Err("Invalid email format".to_string());
    }
    Ok(())
}

/// Password: minimum 8 chars, at least one digit.
pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain at least one number".to_string());
    }
    Ok(())
}

/// Username: 3–50 alphanumeric/underscore characters.
pub fn validate_username(username: &str) -> Result<(), String> {
    let username = username.trim();
    if username.len() < 3 {
        return Err("Username must be at least 3 characters".to_string());
    }
    if username.len() > 50 {
        return Err("Username must be at most 50 characters".to_string());
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Username must contain only letters, numbers, or underscores".to_string());
    }
    Ok(())
}

/// Stellar wallet address: starts with 'G', exactly 56 alphanumeric characters.
pub fn validate_wallet(address: &str) -> Result<(), String> {
    let address = address.trim();
    if address.len() != 56 {
        return Err("Wallet address must be 56 characters".to_string());
    }
    if !address.starts_with('G') {
        return Err("Stellar wallet address must start with 'G'".to_string());
    }
    if !address.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err("Wallet address must be alphanumeric".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_valid() {
        assert!(validate_email("user@example.com").is_ok());
    }
    #[test]
    fn email_no_at() {
        assert!(validate_email("userexample.com").is_err());
    }
    #[test]
    fn password_too_short() {
        assert!(validate_password("abc1").is_err());
    }
    #[test]
    fn password_no_digit() {
        assert!(validate_password("abcdefgh").is_err());
    }
    #[test]
    fn password_valid() {
        assert!(validate_password("Secure1Pass").is_ok());
    }
    #[test]
    fn username_too_short() {
        assert!(validate_username("ab").is_err());
    }
    #[test]
    fn wallet_wrong_length() {
        assert!(validate_wallet("GABC123").is_err());
    }
}