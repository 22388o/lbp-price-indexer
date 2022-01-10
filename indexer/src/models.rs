use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Response<T> {
    pub height: String,
    pub result: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReverseSimulationResponse {
    pub offer_amount: Uint128,
    pub spread_amount: Uint128,
    pub commission_amount: Uint128,
    pub ask_weight: String,
    pub offer_weight: String,
}
