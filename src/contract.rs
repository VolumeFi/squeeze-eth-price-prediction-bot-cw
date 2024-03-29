#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetJobIdResponse, InstantiateMsg, Metadata, PalomaMsg, QueryMsg};
use crate::state::{State, STATE};
use cosmwasm_std::CosmosMsg;
use ethabi::{Contract, Function, Param, ParamType, StateMutability, Token, Uint};
use std::collections::BTreeMap;
use std::str::FromStr;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io::juice-bot-eth-predictor-cw";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        retry_delay: msg.retry_delay,
        job_arb_id: msg.job_arb_id.clone(),
        job_eth_id: msg.job_eth_id.clone(),
        owner: info.sender.clone(),
        metadata: Metadata {
            creator: msg.creator,
            signers: msg.signers,
        },
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("job_arb_id", msg.job_arb_id)
        .add_attribute("job_eth_id", msg.job_eth_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<PalomaMsg>, ContractError> {
    match msg {
        ExecuteMsg::SetEthPaloma {} => execute::set_eth_paloma(deps, info),
        ExecuteMsg::UpdateEthCompass { new_compass } => {
            execute::update_eth_compass(deps, info, new_compass)
        }
        ExecuteMsg::SetRewardToken {
            new_reward_token,
            new_decimals,
        } => execute::set_reward_token(deps, info, new_reward_token, new_decimals),
        ExecuteMsg::SendReward { amount } => execute::send_reward(deps, info, amount),
        ExecuteMsg::SetWinnerList { winner_infos } => {
            execute::set_winner_list(deps, info, winner_infos)
        }

        ExecuteMsg::SetArbPaloma {} => execute::set_arb_paloma(deps, info),
        ExecuteMsg::UpdateArbCompass { new_compass } => {
            execute::update_arb_compass(deps, info, new_compass)
        }
        ExecuteMsg::SetActiveEpoch { epoch_info } => {
            execute::set_active_epoch(deps, info, epoch_info)
        }
    }
}

pub mod execute {
    use super::*;
    use crate::msg::EpochInfo;
    use crate::msg::WinnerInfo;
    use crate::ContractError::Unauthorized;
    use cosmwasm_std::Uint256;
    use ethabi::Address;

    pub fn set_eth_paloma(
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        let state = STATE.load(deps.storage)?;
        if state.owner != info.sender {
            return Err(Unauthorized {});
        }
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "set_paloma".to_string(),
                vec![Function {
                    name: "set_paloma".to_string(),
                    inputs: vec![],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg {
                job_id: state.job_eth_id,
                payload: Binary(
                    contract
                        .function("set_paloma")
                        .unwrap()
                        .encode_input(&[])
                        .unwrap(),
                ),
                metadata: state.metadata,
            }))
            .add_attribute("action", "set_paloma"))
    }

    pub fn update_eth_compass(
        deps: DepsMut,
        info: MessageInfo,
        new_compass: String,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        let state = STATE.load(deps.storage)?;
        if state.owner != info.sender {
            return Err(Unauthorized {});
        }
        let new_compass_address: Address = Address::from_str(new_compass.as_str()).unwrap();
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "update_compass".to_string(),
                vec![Function {
                    name: "update_compass".to_string(),
                    inputs: vec![Param {
                        name: "new_compass".to_string(),
                        kind: ParamType::Address,
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };

        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg {
                job_id: state.job_eth_id,
                payload: Binary(
                    contract
                        .function("update_compass")
                        .unwrap()
                        .encode_input(&[Token::Address(new_compass_address)])
                        .unwrap(),
                ),
                metadata: state.metadata,
            }))
            .add_attribute("action", "update_compass"))
    }

    pub fn send_reward(
        deps: DepsMut,
        info: MessageInfo,
        amount: Uint256,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        let state = STATE.load(deps.storage)?;
        if state.owner != info.sender {
            return Err(Unauthorized {});
        }
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "send_reward".to_string(),
                vec![Function {
                    name: "send_reward".to_string(),
                    inputs: vec![Param {
                        name: "_amount".to_string(),
                        kind: ParamType::Uint(256),
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };

        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg {
                job_id: state.job_eth_id,
                payload: Binary(
                    contract
                        .function("send_reward")
                        .unwrap()
                        .encode_input(&[Token::Uint(Uint::from_big_endian(&amount.to_be_bytes()))])
                        .unwrap(),
                ),
                metadata: state.metadata,
            }))
            .add_attribute("action", "send_reward"))
    }

    pub fn set_winner_list(
        deps: DepsMut,
        info: MessageInfo,
        winner_infos: Vec<WinnerInfo>,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        let state = STATE.load(deps.storage)?;
        if state.owner != info.sender {
            return Err(Unauthorized {});
        }
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "set_winner_list".to_string(),
                vec![Function {
                    name: "set_winner_list".to_string(),
                    inputs: vec![Param {
                        name: "_winner_infos".to_string(),
                        kind: ParamType::Array(Box::new(ParamType::Tuple(vec![
                            ParamType::Address,
                            ParamType::Uint(256),
                        ]))),
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };
        // let mut token_winner_infos: Vec[Token] = vec![];
        let mut token_winner_info: Vec<Token> = vec![];
        for winner_info in winner_infos {
            let mut token_winner_info_element: Vec<Token> = vec![];
            token_winner_info_element.push(Token::Address(
                Address::from_str(winner_info.winner.as_str()).unwrap(),
            ));
            token_winner_info_element.push(Token::Uint(Uint::from_big_endian(
                &winner_info.claimable_amount.to_be_bytes(),
            )));
            token_winner_info.push(Token::Tuple(token_winner_info_element));
        }
        // token_winner_infos.push(Token::Array(token_winner_info));
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg {
                job_id: state.job_eth_id,
                payload: Binary(
                    contract
                        .function("set_winner_list")
                        .unwrap()
                        .encode_input(token_winner_info.as_slice())
                        .unwrap(),
                ),
                metadata: state.metadata,
            }))
            .add_attribute("action", "set_winner_list"))
    }

    pub fn set_reward_token(
        deps: DepsMut,
        info: MessageInfo,
        new_reward_token: String,
        new_decimals: Uint256,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        let state = STATE.load(deps.storage)?;
        if state.owner != info.sender {
            return Err(Unauthorized {});
        }
        let new_reward_token_address =
            Address::from_str(new_reward_token.as_str()).unwrap();
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "set_reward_token".to_string(),
                vec![Function {
                    name: "set_reward_token".to_string(),
                    inputs: vec![
                        Param {
                            name: "_new_reward_token".to_string(),
                            kind: ParamType::Address,
                            internal_type: None,
                        },
                        Param {
                            name: "_new_decimals".to_string(),
                            kind: ParamType::Uint(256),
                            internal_type: None,
                        },
                    ],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };
        let tokens = vec![
            Token::Address(new_reward_token_address),
            Token::Uint(Uint::from_big_endian(
                &new_decimals.to_be_bytes(),
            )),
        ];
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg {
                job_id: state.job_eth_id,
                payload: Binary(
                    contract
                        .function("set_reward_token")
                        .unwrap()
                        .encode_input(tokens.as_slice())
                        .unwrap(),
                ),
                metadata: state.metadata,
            }))
            .add_attribute("action", "set_reward_token"))
    }

    pub fn set_arb_paloma(
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        let state = STATE.load(deps.storage)?;
        if state.owner != info.sender {
            return Err(Unauthorized {});
        }
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "set_paloma".to_string(),
                vec![Function {
                    name: "set_paloma".to_string(),
                    inputs: vec![],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg {
                job_id: state.job_arb_id,
                payload: Binary(
                    contract
                        .function("set_paloma")
                        .unwrap()
                        .encode_input(&[])
                        .unwrap(),
                ),
                metadata: state.metadata,
            }))
            .add_attribute("action", "set_paloma"))
    }

    pub fn update_arb_compass(
        deps: DepsMut,
        info: MessageInfo,
        new_compass: String,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        let state = STATE.load(deps.storage)?;
        if state.owner != info.sender {
            return Err(Unauthorized {});
        }
        let new_compass_address: Address = Address::from_str(new_compass.as_str()).unwrap();
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "update_compass".to_string(),
                vec![Function {
                    name: "update_compass".to_string(),
                    inputs: vec![Param {
                        name: "new_compass".to_string(),
                        kind: ParamType::Address,
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };

        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg {
                job_id: state.job_arb_id,
                payload: Binary(
                    contract
                        .function("update_compass")
                        .unwrap()
                        .encode_input(&[Token::Address(new_compass_address)])
                        .unwrap(),
                ),
                metadata: state.metadata,
            }))
            .add_attribute("action", "update_compass"))
    }

    pub fn set_active_epoch(
        deps: DepsMut,
        info: MessageInfo,
        epoch_info: EpochInfo,
    ) -> Result<Response<PalomaMsg>, ContractError> {
        let state = STATE.load(deps.storage)?;
        if state.owner != info.sender {
            return Err(Unauthorized {});
        }
        #[allow(deprecated)]
        let contract: Contract = Contract {
            constructor: None,
            functions: BTreeMap::from_iter(vec![(
                "set_active_epoch".to_string(),
                vec![Function {
                    name: "set_active_epoch".to_string(),
                    inputs: vec![Param {
                        name: "_epoch_info".to_string(),
                        kind: ParamType::Tuple(vec![
                            ParamType::Uint(256),
                            ParamType::Uint(256),
                            ParamType::Uint(256),
                            ParamType::Uint(256),
                        ]),
                        internal_type: None,
                    }],
                    outputs: Vec::new(),
                    constant: None,
                    state_mutability: StateMutability::NonPayable,
                }],
            )]),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false,
        };

        let mut token_epoch_info_element: Vec<Token> = vec![];
        token_epoch_info_element.push(Token::Uint(Uint::from_big_endian(
            &epoch_info.epoch_id.to_be_bytes(),
        )));
        token_epoch_info_element.push(Token::Uint(Uint::from_big_endian(
            &epoch_info.competition_start.to_be_bytes(),
        )));
        token_epoch_info_element.push(Token::Uint(Uint::from_big_endian(
            &epoch_info.competition_end.to_be_bytes(),
        )));
        token_epoch_info_element.push(Token::Uint(Uint::from_big_endian(
            &epoch_info.entry_cnt.to_be_bytes(),
        )));
        let token_epoch_info = Token::Tuple(token_epoch_info_element);
        Ok(Response::new()
            .add_message(CosmosMsg::Custom(PalomaMsg {
                job_id: state.job_arb_id,
                payload: Binary(
                    contract
                        .function("set_active_epoch")
                        .unwrap()
                        .encode_input(&[token_epoch_info])
                        .unwrap(),
                ),
                metadata: state.metadata,
            }))
            .add_attribute("action", "set_active_epoch"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEthJobId {} => to_json_binary(&query::get_eth_job_id(deps)?),
        QueryMsg::GetArbJobId {} => to_json_binary(&query::get_arb_job_id(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn get_eth_job_id(deps: Deps) -> StdResult<GetJobIdResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetJobIdResponse {
            job_id: state.job_eth_id,
        })
    }

    pub fn get_arb_job_id(deps: Deps) -> StdResult<GetJobIdResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetJobIdResponse {
            job_id: state.job_arb_id,
        })
    }
}
