use ethers_core::types::{Bytes, H256};
use fluct_jsonrpc::{Error, ErrorCode, RpcError};
use serde::{Deserialize, Serialize};

use super::Status;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForkchoiceState {
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

#[derive(Debug, thiserror::Error)]
pub enum ForkChoiceError {
    #[error("Invalid forkchoice state")]
    InvalidForkChoiceState,
    #[error("Invalid payload attributes")]
    InvalidPayloadAttributes,
    #[error("{0}")]
    UnknownRpcError(RpcError),
    #[error(transparent)]
    JsonrpcQueryError(#[from] Error),
    #[error("{0}")]
    Custom(String),
}

impl From<RpcError> for ForkChoiceError {
    fn from(value: RpcError) -> Self {
        match value.code {
            ErrorCode::ServerError(-38002) => ForkChoiceError::InvalidForkChoiceState,
            ErrorCode::ServerError(-38003) => ForkChoiceError::InvalidPayloadAttributes,
            _ => ForkChoiceError::UnknownRpcError(value),
        }
    }
}
