use nutype::nutype;

use crate::UserFacingError;

#[nutype(
    validate(not_empty, len_char_max = 30),
    derive(Debug, Clone, AsRef, Serialize, Deserialize, PartialEq)
)]
pub struct Headline(String);

impl Headline {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for HeadlineError{
    fn formatted_error(&self) -> &'static str {
        match self {
            HeadlineError::NotEmptyViolated => "Headline cannot be empty",
            HeadlineError::LenCharMaxViolated => "Headline is too long. Must be at most 30 characters.",
            
        }
    }
}

#[nutype(
    validate(not_empty, len_char_max = 100),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct Message(String);

impl UserFacingError for MessageError {
    fn formatted_error(&self) -> &'static str {
        match self {
            MessageError::NotEmptyViolated => "Message cannot be empty",
            MessageError::LenCharMaxViolated => "Message must be at most 100 characters",
        }
    }
}