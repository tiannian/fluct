use std::collections::HashMap;

use ethers_core::types::{Bytes, H160, H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimism {
    eip1559_elasticity: u64,
    eip1559_denominator: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainConfig {
    chain_id: u64,
    homestead_block: u64,
    eip150_block: u64,
    eip155_block: u64,
    eip158_block: u64,
    byzantium_block: u64,
    constantinople_block: u64,
    petersburg_block: u64,
    istanbul_block: u64,
    muir_glacier_block: u64,
    berlin_block: u64,
    london_block: u64,
    arrow_glacier_block: u64,
    gray_glacier_block: u64,
    merge_netsplit_block: u64,
    bedrock_block: u64,
    regolith_time: u64,
    terminal_total_difficulty: u64,
    terminal_total_difficulty_passed: bool,
    optimism: Optimism,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Genesis {
    pub config: ChainConfig,
    pub nonce: U256,
    pub timestamp: U256,
    pub extra_data: Bytes,
    pub gas_limit: U256,
    pub difficulty: U256,
    pub mix_hash: H256,
    pub coinbase: H160,
    pub alloc: HashMap<H160, AllocItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AllocItem {
    pub code: Bytes,
    pub storage: HashMap<H256, H256>,
    pub balance: U256,
}

#[cfg(test)]
mod tests {
    use crate::Genesis;

    #[test]
    fn test() {
        let s = include_str!("../genesis.json");

        let r: Genesis = serde_json::from_str(s).unwrap();

        println!("{:#?}", r);
    }
}
