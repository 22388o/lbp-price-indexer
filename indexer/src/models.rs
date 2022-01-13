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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfo {
    Token { contract_addr: String },
    NativeToken { denom: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WeightedAssetInfo {
    pub info: AssetInfo,
    pub start_weight: String,
    pub end_weight: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PairInfo {
    pub asset_infos: [WeightedAssetInfo; 2],
    pub contract_addr: String,
    pub liquidity_token: String,
    pub start_time: u64,
    pub end_time: u64,
    pub description: Option<String>,
}
