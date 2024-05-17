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
    validate(not_empty, len_char_max = 50),
    derive(Debug, Clone, AsRef, Serialize, Deserialize, PartialEq)
)]
pub struct PollHeadline(String);

impl PollHeadline {
    pub const MAX_CHARS: usize = 50;
}

impl UserFacingError for PollHeadlineError{
    fn formatted_error(&self) -> &'static str {
        match self {
            PollHeadlineError::NotEmptyViolated => "Headline cannot be empty",
            PollHeadlineError::LenCharMaxViolated => "Headline is too long. Must be at most 50 characters.",
            
        }
    }
}

#[nutype(
    validate(not_empty, len_char_max = 100),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct Message(String);
impl Message {
    pub const  MAX_CHARS: usize = 100;
}

impl UserFacingError for MessageError {
    fn formatted_error(&self) -> &'static str {
        match self {
            MessageError::NotEmptyViolated => "Message cannot be empty",
            MessageError::LenCharMaxViolated => "Message must be at most 100 characters",
        }
    }
}

#[nutype(
    validate(not_empty, len_char_max = 60),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct Caption(String);
impl Caption {
    pub const  MAX_CHARS: usize = 60;
}

impl UserFacingError for CaptionError {
    fn formatted_error(&self) -> &'static str {
        match self {
            CaptionError::NotEmptyViolated => "Caption cannot be empty",
            CaptionError::LenCharMaxViolated => "Caption must be at most 60 characters",
        }
    }
}

#[nutype(
    validate(not_empty, len_char_max = 80),
    derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)
)]
pub struct PollChoiceDescription(String);
impl PollChoiceDescription {
    pub const  MAX_CHARS: usize = 80;
}

impl UserFacingError for PollChoiceDescriptionError {
    fn formatted_error(&self) -> &'static str {
        match self {
            PollChoiceDescriptionError::NotEmptyViolated => "Poll choice cannot be empty",
            PollChoiceDescriptionError::LenCharMaxViolated => "Poll choice must be at most 60 characters",
        }
    }
}