#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetLunarResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

use zodiatic_bindings::{Lunar, ZodiaticMsg, ZodiaticQuerier, ZodiaticQuery};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-zodiatic";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ZodiaticQuery>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<ZodiaticQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ZodiaticMsg>, ContractError> {
    match msg {
        ExecuteMsg::CreateLunar { yyyymmdd, lunar } => create_lunar(info.sender, yyyymmdd, lunar),
    }
}

pub fn create_lunar(
    creator: Addr,
    yyyymmdd: u64,
    lunar: Lunar,
) -> Result<Response<ZodiaticMsg>, ContractError> {
    Ok(Response::new()
        .add_attribute("method", "create_lunar")
        .add_message(ZodiaticMsg::CreateLunar {
            creator: creator.to_string(),
            yyyymmdd,
            lunar,
        }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ZodiaticQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLunar { yyyymmdd } => to_binary(&get_lunar(deps, yyyymmdd)),
    }
}

fn get_lunar(deps: Deps<ZodiaticQuery>, yyyymmdd: u64) -> GetLunarResponse {
    let querier = ZodiaticQuerier::new(&deps.querier);
    let response = querier.lunar(yyyymmdd).unwrap();

    GetLunarResponse {
        lunar: response.lunar,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use zodiatic_bindings_test::mock::mock_dependencies;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
