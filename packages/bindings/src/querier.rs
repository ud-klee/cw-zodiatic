use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use crate::query::{LunarResponse, ZodiaticQuery};

/// This is a helper wrapper to easily use our custom queries
pub struct ZodiaticQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ZodiaticQuery>,
}

impl<'a> ZodiaticQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<ZodiaticQuery>) -> Self {
        ZodiaticQuerier { querier }
    }

    pub fn lunar(&self, yyyymmdd: u64) -> StdResult<LunarResponse> {
        let lunar_query = ZodiaticQuery::lunar(yyyymmdd);
        let request: QueryRequest<ZodiaticQuery> = ZodiaticQuery::into(lunar_query);
        self.querier.query(&request)
    }
}
