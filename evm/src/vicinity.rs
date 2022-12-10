use primitive_types::{H160, U256};

#[derive(Debug, Default)]
pub struct CoreVicinity {
    pub gas_price: U256,
    pub block_height: u64,
    pub block_coinbase: H160,
    pub block_timestamp: U256,
    pub block_difficulty: U256,
    pub block_gas_limit: U256,
    pub block_base_fee_per_gas: U256,
    pub chain_id: U256,
}
