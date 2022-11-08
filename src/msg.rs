use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Decrement{},
    DecrementBy {amount:i32},
    IncrementBy {amount:i32},
    UpdateState {new_value:i32},
    Reset { count: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
    
    #[returns(bool)]
    HasReset {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
