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
    GetLunar {
        yyyymmdd: u64,
    },
    FindLunar {
        year: u32,
        predicates: Vec<Predicate>,
    },
}

// We define a custom struct for each query response

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetLunarResponse {
    pub lunar: Lunar,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FindLunarResponse {
    pub result: Vec<Lunar>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Predicate {
    And(Vec<(String, String)>),
    AndNot(Vec<(String, String)>),
}

impl Predicate {
    pub fn and(any_of: Vec<&str>) -> Self {
        Self::And(into_tuples(any_of))
    }

    pub fn and_not(any_of: Vec<&str>) -> Self {
        Self::AndNot(into_tuples(any_of))
    }
}

fn into_tuples(any_of: Vec<&str>) -> Vec<(String, String)> {
    any_of
        .iter()
        .map(|kv| {
            let fv = kv.splitn(2, "=").collect::<Vec<_>>();
            let (field, value) = (fv[0], fv[1]);
            (field.to_string(), value.to_string())
        })
        .collect::<Vec<_>>()
}
