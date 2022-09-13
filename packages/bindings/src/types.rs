use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Lunar {
    pub date: String,
    pub lunar_number: (u64, u64, u64),
    pub lunar: String,
    pub eight_words: String,
    pub god_direction: String,
    pub good_for: String,
    pub bad_for: String,
}

impl Lunar {
    pub fn new(
        date: &str,
        lunar_number: (u64, u64, u64),
        lunar: &str,
        eight_words: &str,
        god_direction: &str,
        good_for: &str,
        bad_for: &str,
    ) -> Lunar {
        Lunar {
            date: date.into(),
            lunar_number,
            lunar: lunar.into(),
            eight_words: eight_words.into(),
            god_direction: god_direction.into(),
            good_for: good_for.into(),
            bad_for: bad_for.into(),
        }
    }
}

impl Default for Lunar {
    fn default() -> Self {
        Lunar::new("", (0, 0, 0), "", "", "", "", "")
    }
}
