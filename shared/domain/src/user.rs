use nutype::nutype;

#[nutype(
    validate(not_empty, len_char_min = 3, len_char_max = 30),
    derive(Debug, Clone, AsRef, Serialize, Deserialize, PartialEq)
)]

pub struct Username(String);

#[nutype(
    validate(not_empty, len_char_min = 8),
    derive(Clone, AsRef, Serialize, Deserialize, PartialEq)
)]

pub struct Password(String);