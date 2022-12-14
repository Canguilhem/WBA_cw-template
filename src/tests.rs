#[cfg(test)]

// TODO add missing tests

/*  Proper initialization Ok
    ExecuteMsg::Reset Ok
    ExecuteMsg::DecrementBy Ok
    ExecuteMsg::IncrementBy Ok
    ExecuteMsg::UpdateState
    QueryMsg::GetCount
    QueryMsg::HasReset
*/

mod tests {
    
    use crate::ContractError;
    use crate::contract::{instantiate, query, execute};
    use crate::msg::{InstantiateMsg, QueryMsg, GetCountResponse, ExecuteMsg};

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};


#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg { count: 17 };
    let info = mock_info("creator", &coins(1000, "earth"));
    // we can just call .unwrap() to assert this was a success
    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    // it worked, let's query the state
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: GetCountResponse = from_binary(&res).unwrap();
    assert_eq!(17, value.count);
}

#[test]
fn increment() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg { count: 17 };
    let info = mock_info("creator", &coins(2, "token"));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    // beneficiary can release it
    let info = mock_info("anyone", &coins(2, "token"));
    let msg = ExecuteMsg::Increment {};
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    // should increase counter by 1
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: GetCountResponse = from_binary(&res).unwrap();
    assert_eq!(18, value.count);
}

#[test]
fn increment_by() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg { count: 17 };
    let info = mock_info("creator", &coins(2, "token"));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    // beneficiary can release it
    let info = mock_info("anyone", &coins(2, "token"));
    let msg = ExecuteMsg::IncrementBy { amount: 17 };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    // should increase counter by 17
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: GetCountResponse = from_binary(&res).unwrap();
    assert_eq!(34, value.count);
}

#[test]
fn decrement() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg { count: 17 };
    let info = mock_info("creator", &coins(2, "token"));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    // beneficiary can release it
    let info = mock_info("anyone", &coins(2, "token"));
    let msg = ExecuteMsg::Decrement {};
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    // should decrease counter by 1
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: GetCountResponse = from_binary(&res).unwrap();
    assert_eq!(16, value.count);
}

#[test]
fn decrement_by() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg { count: 17 };
    let info = mock_info("creator", &coins(2, "token"));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    // beneficiary can release it
    let info = mock_info("anyone", &coins(2, "token"));
    let msg = ExecuteMsg::DecrementBy { amount: 17 };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    // should decrease counter by 17
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: GetCountResponse = from_binary(&res).unwrap();
    assert_eq!(0, value.count);
}

#[test]
fn reset() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg { count: 17 };
    let info = mock_info("creator", &coins(2, "token"));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    // beneficiary can release it
    let unauth_info = mock_info("anyone", &coins(2, "token"));
    let msg = ExecuteMsg::Reset { count: 5 };
    let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Must return unauthorized error"),
    }
    // only the original creator can reset the counter
    let auth_info = mock_info("creator", &coins(2, "token"));
    let msg = ExecuteMsg::Reset { count: 5 };
    let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();
    // should now be 5
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: GetCountResponse = from_binary(&res).unwrap();
    assert_eq!(5, value.count);
}

#[test]
fn has_reset() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg { count: 17 };
    let info = mock_info("creator", &coins(2, "token"));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    
    // beneficiary can release it
    let auth_info = mock_info("creator", &coins(2, "token"));
    let msg = ExecuteMsg::Reset { count: 5 };
    let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();
    
    // should now be 5
    let res = query(deps.as_ref(), mock_env(), QueryMsg::HasReset {}).unwrap();
    let value: bool = from_binary(&res).unwrap();
    
    //  no sure what to test for the HasReset query as it seems that reset can set the value to whatever it wants
    //  QueryMsg::HasReset implem might be wrong
    assert_eq!(true, value);

}


}