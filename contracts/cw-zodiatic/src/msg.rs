use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use zodiatic_bindings::Lunar;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateLunar { yyyymmdd: u64, lunar: Lunar },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetLunar { yyyymmdd: u64 },
}

// We define a custom struct for each query response

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetLunarResponse {
    pub lunar: Lunar,
}
