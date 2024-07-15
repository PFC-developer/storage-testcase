use cosmwasm_std::{Addr, Api, Order, StdResult, Uint128};
use cosmwasm_std::testing::mock_dependencies;
use cw_storage_plus::{Index, IndexedMap, IndexList, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub(crate) const UNBOND_KEY_V101: &str = "unbond_101";
pub(crate) const UNBOND_KEY_USER_V101: &str = "unbond_owner_101";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UnbondRequest {
    /// ID of the batch
    pub id: u64,
    /// The user's address
    pub user: Addr,
    /// The user's share in the batch
    pub shares: Uint128,
}
pub struct UnbondRequestsIndexes<'a> {
    // pk goes to second tuple element
    pub user: MultiIndex<'a, String, UnbondRequest, (u64,Addr)>,
}

impl<'a> IndexList<UnbondRequest> for UnbondRequestsIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<UnbondRequest>> + '_> {
        let v: Vec<&dyn Index<UnbondRequest>> = vec![&self.user];
        Box::new(v.into_iter())
    }
}

pub fn unbond_requests_user_idx(_pk: &[u8], d: &UnbondRequest) -> String {
    d.user.to_string()
}

pub fn unbond_requests<'a>()
    -> IndexedMap< (u64, &'a str), UnbondRequest, UnbondRequestsIndexes<'a>> {
    IndexedMap::new(
        UNBOND_KEY_V101,
        UnbondRequestsIndexes {
            user: MultiIndex::new(unbond_requests_user_idx, UNBOND_KEY_V101, UNBOND_KEY_USER_V101),
        },
    )
}


fn main() {

    let mut deps = mock_dependencies();

    let user_addr = deps.api.addr_make("testing");

    unbond_requests()
        .save(
            deps.as_mut().storage,
            (128u64, user_addr.as_str()),
            &UnbondRequest {
                id: 128u64,
                user: user_addr.clone(), //Addr::unchecked(contract),
                shares: Uint128::new(18084001808),
            },
        )
        .unwrap();

    let  mut_deps = deps.as_mut();

    let elements = unbond_requests()
        .range(mut_deps.storage, None, None, Order::Ascending)
        .take(10)
        .map(|item| {
            let (_, v) = item?;
            Ok(v.into())
        })
        .collect::<StdResult<Vec<UnbondRequest>>>()
        .unwrap();
    eprintln!("{:?}", elements);
    let elements = unbond_requests()
        .idx
        .user
        .prefix(user_addr.to_string())
        //  .prefix(contract.to_string())
        .range(mut_deps.storage, None, None, Order::Ascending)
        .take(100)
        .map(|item| {
            let (_, v) = item?;
            Ok(v.into())
        })
        .collect::<StdResult<Vec<UnbondRequest>>>()
        .unwrap();

    assert_eq!(elements.len(), 1);
}
