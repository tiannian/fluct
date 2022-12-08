use ethereum::PartialHeader;
use primitive_types::{H160, U256};

pub struct CoreVicinity {
    pub gas_price: U256,
    pub origin: H160,
    pub block_height: u64,
    pub block_coinbase: H160,
    pub block_timestamp: U256,
    pub block_difficulty: U256,
    pub block_gas_limit: U256,
    pub block_base_fee_per_gas: U256,
    pub chain_id: U256,
}

impl CoreVicinity {
    pub fn from_partial_header(
        header: &PartialHeader,
        gas_price: U256,
        origin: H160,
        chain_id: U256,
        block_base_fee_per_gas: U256,
    ) -> Self {
        Self {
            gas_price,
            origin,
            block_height: header.number.as_u64(),
            block_coinbase: header.beneficiary,
            block_timestamp: header.timestamp.into(),
            block_difficulty: header.difficulty,
            block_gas_limit: header.gas_limit,
            block_base_fee_per_gas,
            chain_id,
        }
    }
}
