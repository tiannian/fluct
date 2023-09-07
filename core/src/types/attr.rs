use ethers_core::types::{H160, H256, U256};
use serde::{Deserialize, Serialize};

/// Attributes of payload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadAttributes<T> {
    pub timestamp: U256,
    pub prev_randao: H256,
    pub suggested_fee_recipient: H160,
    pub transactions: Vec<T>,
    pub gas_limit: Option<U256>,
}

impl<T> PayloadAttributes<T> {
    pub fn into_other_tx<O>(self, txs: Vec<O>) -> PayloadAttributes<O> {
        PayloadAttributes {
            timestamp: self.timestamp,
            prev_randao: self.prev_randao,
            suggested_fee_recipient: self.suggested_fee_recipient,
            transactions: txs,
            gas_limit: self.gas_limit,
        }
    }
}
