use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CosmosMsg, CustomMsg};

use crate::types::Lunar;

/// A number of Custom messages that can call into the Zodiatic bindings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ZodiaticMsg {
    CreateLunar {
        creator: String,
        yyyymmdd: u64,
        lunar: Lunar,
    },
}

impl ZodiaticMsg {
    pub fn create_lunar(creator: String, yyyymmdd: u64, lunar: Lunar) -> Self {
        ZodiaticMsg::CreateLunar {
            creator,
            yyyymmdd,
            lunar,
        }
    }
}

impl From<ZodiaticMsg> for CosmosMsg<ZodiaticMsg> {
    fn from(msg: ZodiaticMsg) -> CosmosMsg<ZodiaticMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ZodiaticMsg {}
