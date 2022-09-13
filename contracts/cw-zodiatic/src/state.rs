use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

// "year[field=value]" -> bitmap
pub const INDEX: Map<String, Vec<u32>> = Map::new("index");

// year -> [yyyymmdd, ...]
pub const KEYS: Map<u32, Vec<u64>> = Map::new("keys");
