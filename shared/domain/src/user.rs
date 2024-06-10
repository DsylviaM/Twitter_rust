// use nutype::nutype;
use once_cell::sync::OnceCell;
use regex::Regex;
use nutype::nutype; // Импортируем сам атрибут nutype


// use nutype::validators::function; // Импортируем валидатор function 



use serde::{Deserialize, Serialize};

use crate::UserFacingError;

// NOTE Using the `nutype` crate to easily add field validation
#[nutype(
    validate(not_empty, len_char_min = 3, len_char_max = 30),
    derive(Debug, Clone, AsRef, Serialize, Deserialize, PartialEq)
)]
pub struct Username(String);
// NOTE The nutype validate macro automatically generates the UsernameError enum
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

#[nutype(
    validate(len_char_max = 30),
    derive(Debug, Clone, AsRef, Serialize, Deserialize, PartialEq)
)]
pub struct DisplayName(String);

impl DisplayName {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for DisplayNameError{
    fn formatted_error(&self) -> &'static str {
        match self {
            DisplayNameError::LenCharMaxViolated => "Display name is too long. Must be at most 30 characters.",
            
            
        }
    }
}

static EMAIL_REGEX: OnceCell<EmailRegex> = OnceCell::new();

#[derive(Debug)]
pub struct EmailRegex(Regex);

impl EmailRegex {
    pub fn global() -> &'static Self {
        EMAIL_REGEX.get().expect("email regex is not initialized")
    }

    pub fn init() -> Self {
        Self(regex::Regex::new(r#"^\S+@\S+\.\S{1,64}$"#).unwrap())
    }

    pub fn is_valid<T: AsRef<str>>(&self, email: T) -> bool {
        self.0.is_match(email.as_ref())
    }
}

fn is_valid_email(email: &str) -> bool {
     let email_regex = EMAIL_REGEX.get_or_init(EmailRegex::init);

     email_regex.is_valid(email)
   // todo!();
}

#[nutype(
    validate(predicate = is_valid_email), 
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct Email(String);



// NOTE The nutype validate macro automatically generates the EmailError enum
impl UserFacingError for EmailError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::PredicateViolated => "Email is not valid. Format: your_name@example.com",
        }
    }
}


