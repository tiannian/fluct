use ethers_core::types::{Bytes, H160, H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayload<T> {
    parent_hash: H256,
    fee_recipient: H160,
    state_root: H256,
    receipts_root: H256,
    logs_bloom: Bytes,
    prev_randao: H256,
    block_number: U256,
    gas_limit: U256,
    gas_used: U256,
    timestamp: U256,
    extra_data: Bytes,
    base_fee_per_gas: U256,
    block_hash: H256,
    transactions: Vec<T>,
}
