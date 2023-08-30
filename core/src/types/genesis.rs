use ethers_core::types::{Bytes, H256, U256};
use serde::{Deserialize, Serialize};

use crate::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Genesis<T, E> {
    pub chain_id: u64,
    pub earliest_block_height: u64,
    pub earliest_block_hash: H256,
    pub extra_data: Bytes,
    pub block_gas_limit: U256,
    pub block_block_size: u64,
    pub transactions: Vec<T>,
    pub timestamp: u64,
    pub execution: E,
}

impl<E> From<(Genesis<Bytes, E>, Vec<Transaction>)> for Genesis<Transaction, E> {
    fn from(value: (Genesis<Bytes, E>, Vec<Transaction>)) -> Self {
        Self {
            chain_id: value.0.chain_id,
            earliest_block_height: value.0.earliest_block_height,
            earliest_block_hash: value.0.earliest_block_hash,
            extra_data: value.0.extra_data,
            block_gas_limit: value.0.block_gas_limit,
            block_block_size: value.0.block_block_size,
            transactions: value.1,
            timestamp: value.0.timestamp,
            execution: value.0.execution,
        }
    }
}
