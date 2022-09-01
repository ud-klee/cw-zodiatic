use anyhow::{bail, Result as AnyResult};

use schemars::JsonSchema;
use serde::de::DeserializeOwned;

use std::cmp::max;
use std::fmt::Debug;

use std::ops::{Deref, DerefMut};
use thiserror::Error;

use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{
    Addr, Api, Binary, BlockInfo, CustomQuery, Empty, Querier, QuerierResult, StdError, Storage,
};
use cw_multi_test::{
    App, AppResponse, BankKeeper, BasicAppBuilder, CosmosRouter, Module, WasmKeeper,
};

use zodiatic_bindings::{ZodiaticMsg, ZodiaticQuery};

pub struct ZodiaticModule {}

/// How many seconds per block
/// (when we increment block.height, use this multiplier for block.time)
pub const BLOCK_TIME: u64 = 5;

impl Module for ZodiaticModule {
    type ExecT = ZodiaticMsg;
    type QueryT = ZodiaticQuery;
    type SudoT = Empty;

    // Builds a mock rust implementation of the expected Zodiatic functionality for testing
    fn execute<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        _msg: ZodiaticMsg,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!("sudo not implemented for ZodiaticModule")
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!("sudo not implemented for ZodiaticModule")
    }

    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _request: ZodiaticQuery,
    ) -> anyhow::Result<Binary> {
        bail!("sudo not implemented for ZodiaticModule")
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ZodiaticError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] cosmwasm_std::OverflowError),

    /// Remove this to let the compiler find all TODOs
    #[error("Not yet implemented (TODO)")]
    Unimplemented,
}

pub type ZodiaticAppWrapped =
    App<BankKeeper, MockApi, MockStorage, ZodiaticModule, WasmKeeper<ZodiaticMsg, ZodiaticQuery>>;

pub struct ZodiaticApp(ZodiaticAppWrapped);

impl Deref for ZodiaticApp {
    type Target = ZodiaticAppWrapped;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ZodiaticApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Querier for ZodiaticApp {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        self.0.raw_query(bin_request)
    }
}

impl Default for ZodiaticApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ZodiaticApp {
    pub fn new() -> Self {
        Self(
            BasicAppBuilder::<ZodiaticMsg, ZodiaticQuery>::new_custom()
                .with_custom(ZodiaticModule {})
                .build(|_router, _, _storage| {
                    // router.custom.set_owner(storage, &owner).unwrap();
                }),
        )
    }

    pub fn block_info(&self) -> BlockInfo {
        self.0.block_info()
    }

    /// This advances BlockInfo by given number of blocks.
    /// It does not do any callbacks, but keeps the ratio of seconds/block
    pub fn advance_blocks(&mut self, blocks: u64) {
        self.update_block(|block| {
            block.time = block.time.plus_seconds(BLOCK_TIME * blocks);
            block.height += blocks;
        });
    }

    /// This advances BlockInfo by given number of seconds.
    /// It does not do any callbacks, but keeps the ratio of seconds/block
    pub fn advance_seconds(&mut self, seconds: u64) {
        self.update_block(|block| {
            block.time = block.time.plus_seconds(seconds);
            block.height += max(1, seconds / BLOCK_TIME);
        });
    }

    /// Simple iterator when you don't care too much about the details and just want to
    /// simulate forward motion.
    pub fn next_block(&mut self) {
        self.advance_blocks(1)
    }
}
