use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Lunar {
    date: String,
    lunar_number: (u64, u64, u64),
    lunar: String,
    eight_words: String,
    god_direction: String,
    good_for: String,
    bad_for: String,
}
