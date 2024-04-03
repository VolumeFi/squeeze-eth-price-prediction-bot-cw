# Juice bot eth predictor CosmWasm smart contract

This is a CosmWasm smart contract to manage ETH price prediction game vyper contract on EVM chain (ARB & ETH) written in Vyper.

## ExecuteMsg

### SetEthPaloma

Run `set_paloma` function on CompetitionEth Vyper smart contract to register this contract address data in the Vyper contract.

### UpdateEthCompass

### SetRewardToken

### SendReward

### SetWinnerList

### SetArbPaloma

### UpdateArbCompass

### SetActiveEpoch

## QueryMsg

### GetEthJobId

Get `job_eth_id` of Paloma message to run functions on a CompetitionEth Vyper smart contract.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

### GetArbJobId

Get `job_arb_id` of Paloma message to run functions on a CompetitionArb Vyper smart contract.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

#### Response

| Key    | Type   | Description      |
|--------|--------|------------------|
| job_id | String | Job Id on Paloma |

## Structs

### WinnerInfo

### EpochInfo


