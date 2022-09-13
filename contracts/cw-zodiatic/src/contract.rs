#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Storage,
};
use cw2::set_contract_version;

use crate::bitmap::BitMap;
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, FindLunarResponse, GetLunarResponse, InstantiateMsg, Predicate, QueryMsg,
};
use crate::state::{State, INDEX, KEYS, STATE};

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
    deps: DepsMut<ZodiaticQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ZodiaticMsg>, ContractError> {
    match msg {
        ExecuteMsg::CreateLunar { yyyymmdd, lunar } => {
            create_lunar(deps, info.sender, yyyymmdd, lunar)
        }
    }
}

pub fn create_lunar(
    deps: DepsMut<ZodiaticQuery>,
    creator: Addr,
    yyyymmdd: u64,
    lunar: Lunar,
) -> Result<Response<ZodiaticMsg>, ContractError> {
    let year: u32 = (yyyymmdd / 10000).try_into().unwrap();
    let keys = KEYS.update(deps.storage, year, |val| -> Result<_, ContractError> {
        if let Some(mut val) = val {
            val.push(yyyymmdd);
            Ok(val)
        } else {
            Ok(vec![yyyymmdd])
        }
    })?;

    let row_id = keys.len() - 1;

    update_index(deps.storage, year, "good_for", &lunar.good_for, row_id)?;
    update_index(deps.storage, year, "bad_for", &lunar.bad_for, row_id)?;

    Ok(Response::new()
        .add_attribute("method", "create_lunar")
        .add_message(ZodiaticMsg::CreateLunar {
            creator: creator.to_string(),
            yyyymmdd,
            lunar,
        }))
}

fn get_key(year: u32, field: &str, value: &str) -> String {
    format!("{}[{}={}]", year, field, value)
}

fn update_index(
    storage: &mut dyn Storage,
    year: u32,
    field: &str,
    values: &String,
    row_id: usize,
) -> Result<(), ContractError> {
    for value in values.split_ascii_whitespace() {
        let key = get_key(year, field, value);
        INDEX.update(storage, key, |bitmap| -> Result<_, ContractError> {
            let mut bm = if let Some(bitmap) = bitmap {
                BitMap::from_vec(bitmap)
            } else {
                BitMap::new()
            };
            bm.set(row_id);
            Ok(bm.into_vec())
        })?;
    }
    Ok(())
}

fn _try_get_index(
    deps: Deps<ZodiaticQuery>,
    year: u32,
    field: &str,
    value: &str,
) -> Result<Option<BitMap>, ContractError> {
    let key = get_key(year, field, value);
    let vec = INDEX.may_load(deps.storage, key)?;
    Ok(vec.map(|vec| BitMap::from_vec(vec)))
}

fn _get_index_or_default(deps: Deps<ZodiaticQuery>, year: u32, field: &str, value: &str) -> BitMap {
    _try_get_index(deps, year, field, value)
        .unwrap()
        .unwrap_or_default()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ZodiaticQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLunar { yyyymmdd } => to_binary(&get_lunar(deps, yyyymmdd)?),
        QueryMsg::FindLunar { year, predicates } => to_binary(&find_lunar(deps, year, predicates)?),
    }
}

fn get_lunar(deps: Deps<ZodiaticQuery>, yyyymmdd: u64) -> StdResult<GetLunarResponse> {
    let querier = ZodiaticQuerier::new(&deps.querier);
    let response = querier.lunar(yyyymmdd)?;

    Ok(GetLunarResponse {
        lunar: response.lunar,
    })
}

fn match_any(deps: Deps<ZodiaticQuery>, year: u32, any_of: Vec<(String, String)>) -> BitMap {
    let mut bm = BitMap::new();
    any_of
        .iter()
        .map(|(field, value)| _get_index_or_default(deps, year, field, value))
        .for_each(|next| {
            bm.or(next);
        });
    bm
}

fn try_find_lunar(
    deps: Deps<ZodiaticQuery>,
    year: u32,
    predicates: Vec<Predicate>,
) -> Option<Vec<u64>> {
    match KEYS.load(deps.storage, year) {
        Ok(keys) => {
            let mut bv = BitMap::ones(366);

            for p in predicates {
                match p {
                    Predicate::And(any_of) => {
                        bv.and(match_any(deps, year, any_of));
                    }
                    Predicate::AndNot(any_of) => {
                        bv.xor(match_any(deps, year, any_of));
                    }
                };
            }
            Some(
                bv.iter_ones()
                    .map(|i| keys.get(i).unwrap())
                    .copied()
                    .collect::<Vec<_>>(),
            )
        }
        _ => None,
    }
}

fn find_lunar(
    deps: Deps<ZodiaticQuery>,
    year: u32,
    predicates: Vec<Predicate>,
) -> StdResult<FindLunarResponse> {
    let mut result: Vec<Lunar> = vec![];
    let keys = try_find_lunar(deps, year, predicates);
    let querier = ZodiaticQuerier::new(&deps.querier);

    match keys {
        Some(keys) => {
            keys.iter().for_each(|yyyymmdd| {
                let res = querier.lunar(*yyyymmdd);
                if let Ok(res) = res {
                    result.push(res.lunar);
                }
            });
            Ok(FindLunarResponse { result })
        }
        _ => Err(StdError::not_found(year.to_string())),
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

    #[test]
    fn test_create_lunar() {
        let mut deps = mock_dependencies();

        create_lunar(
            deps.as_mut(),
            Addr::unchecked("creator"),
            220101,
            Lunar::new(
                "2022-01-01",
                (2021, 11, 29),
                "二零二一 辛醜[牛]年 冬月大廿九",
                "辛醜 庚子 甲寅 甲子",
                "喜神東北 財神東北 福神正北 陽貴西南 陰貴東北",
                "沐浴 上表章 進人口 豎柱上梁 開市 納財 掃舍宇 牧養 破土 啟攢 慶賜 解除 訴訟",
                "祭祀 冠帶 開渠 穿井 畋獵 苫蓋 遠回 乘船渡水 取魚",
            ),
        )
        .unwrap();

        create_lunar(
            deps.as_mut(),
            Addr::unchecked("creator"),
            220131,
            Lunar::new(
                "2022-01-31",
                (2021, 12, 29),
                "二零二一 辛醜[牛]年 臘月小廿九",
                "辛醜 辛醜 甲申 甲子",
                "喜神東北 財神東北 福神正北 陽貴西南 陰貴東北",
                "沐浴 上表章 上官 進人口 豎柱上梁 開市 納財 掃舍宇 牧養 安葬 啟攢 施恩 納畜 招賢",
                "冠帶 修置產室 開渠 穿井 安碓磑 平治道塗 破屋壞垣 畋獵 苫蓋 補垣 取魚 築堤防",
            ),
        )
        .unwrap();

        let res = try_find_lunar(
            deps.as_ref(),
            22,
            vec![
                Predicate::and(vec!["good_for=慶賜", "good_for=安葬"]),
                Predicate::and_not(vec!["bad_for=乘船渡水"]),
            ],
        )
        .unwrap();

        assert_eq!(1, res.len());
        assert_eq!(220131, res[0]);
    }
}
