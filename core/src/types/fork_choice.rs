use ethers_core::types::{Bytes, H256};
use serde::{Deserialize, Serialize};

use super::Status;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForkChoiceState {
    pub head_block_hash: H256,
    pub safe_block_hash: H256,
    pub finalized_block_hash: H256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadStatus {
    status: Status,
    latest_valid_hash: Option<H256>,
    validation_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForkChoiceResult {
    payload_status: PayloadStatus,
    payload_id: Option<Bytes>,
}
