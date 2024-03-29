use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, CustomMsg, Uint256};

#[cw_serde]
pub struct InstantiateMsg {
    pub retry_delay: u64,
    pub job_arb_id: String,
    pub job_eth_id: String,
    pub creator: String,
    pub signers: Vec<String>,
}

#[cw_serde]
pub struct WinnerInfo {
    pub winner: String,
    pub claimable_amount: Uint256,
}

#[cw_serde]
pub struct EpochInfo {
    pub epoch_id: Uint256,
    pub competition_start: Uint256,
    pub competition_end: Uint256,
    pub entry_cnt: Uint256,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetEthPaloma {},
    UpdateEthCompass {
        new_compass: String,
    },
    SetRewardToken {
        new_reward_token: String,
        new_decimals: Uint256,
    },
    SendReward {
        amount: Uint256,
    },
    SetWinnerList {
        winner_infos: Vec<WinnerInfo>,
    },
    SetArbPaloma {},
    UpdateArbCompass {
        new_compass: String,
    },
    SetActiveEpoch {
        epoch_info: EpochInfo,
    },
}

#[cw_serde]
#[derive(Eq)]
pub struct Metadata {
    pub creator: String,
    pub signers: Vec<String>,
}

/// Message struct for cross-chain calls.
#[cw_serde]
pub struct PalomaMsg {
    /// The ID of the paloma scheduled job to run.
    pub job_id: String,
    /// The payload, ABI encoded for the target chain.
    pub payload: Binary,
    /// Metadata
    pub metadata: Metadata,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetJobIdResponse)]
    GetEthJobId {},
    #[returns(GetJobIdResponse)]
    GetArbJobId {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetJobIdResponse {
    pub job_id: String,
}

impl CustomMsg for PalomaMsg {}
