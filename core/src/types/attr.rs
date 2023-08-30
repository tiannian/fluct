use ethers_core::types::{H160, H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadAttributesV1<T> {
    pub timestamp: U256,
    pub prev_randao: H256,
    pub suggested_fee_recipient: H160,
    pub transactions: Vec<T>,
    pub gas_limit: Option<U256>,
}
