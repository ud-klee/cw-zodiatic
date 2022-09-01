use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CustomQuery;

use crate::types::Lunar;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ZodiaticQuery {
    Lunar { yyyymmdd: u64 },
}

impl CustomQuery for ZodiaticQuery {}

impl ZodiaticQuery {
    pub fn lunar(yyyymmdd: u64) -> Self {
        ZodiaticQuery::Lunar { yyyymmdd }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LunarResponse {
    pub lunar: Lunar,
}
