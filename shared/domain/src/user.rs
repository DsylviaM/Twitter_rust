use nutype::nutype;

use crate::UserFacingError;

#[nutype(
    validate(not_empty, len_char_min = 3, len_char_max = 30),
    derive(Debug, Clone, AsRef, Serialize, Deserialize, PartialEq)
)]
pub struct Username(String);

impl UserFacingError for UsernameError{
    fn formatted_error(&self) -> &'static str {
        match self {
            UsernameError::NotEmptyViolated => "User name cannot be empty",
            UsernameError::LenCharMinViolated => "User name is too short. Must be at least 3 characters.",
            UsernameError::LenCharMaxViolated => "User name is too long. Must be at most 30 characters.",
            
        }
    }
}

#[nutype(
    validate(not_empty, len_char_min = 8),
    derive(Clone, AsRef, Serialize, Deserialize, PartialEq)
)]

pub struct Password(String);

impl UserFacingError for PasswordError{
    fn formatted_error(&self) -> &'static str {
        match self {
            PasswordError::NotEmptyViolated => "Password cannot be empty",
            PasswordError::LenCharMinViolated => "Password is too short. Must be at least 3 characters.",
        }
    }
}

