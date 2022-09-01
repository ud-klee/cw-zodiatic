use std::marker::PhantomData;

use cosmwasm_std::{
    testing::{MockApi, MockStorage},
    OwnedDeps, Querier,
};
use zodiatic_bindings::ZodiaticQuery;

use crate::ZodiaticApp;

fn mock_dependencies_with_custom_quierier<Q: Querier>(
    querier: Q,
) -> OwnedDeps<MockStorage, MockApi, Q, ZodiaticQuery> {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier,
        custom_query_type: PhantomData,
    }
}

pub fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, ZodiaticApp, ZodiaticQuery> {
    let custom_querier = ZodiaticApp::new();
    mock_dependencies_with_custom_quierier(custom_querier)
}
