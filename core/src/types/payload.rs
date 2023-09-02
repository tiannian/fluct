use ethers_core::types::{Bytes, H160, H256, U256};
use fluct_jsonrpc::{Error, ErrorCode, RpcError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayload<T> {
    pub parent_hash: H256,
    pub fee_recipient: H160,
    pub state_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: Bytes,
    pub prev_randao: H256,
    pub block_number: U256,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub timestamp: U256,
    pub extra_data: Bytes,
    pub base_fee_per_gas: U256,
    pub block_hash: H256,
    pub transactions: Vec<T>,
}

impl<T> ExecutionPayload<T> {
    pub fn into_other_tx<O>(self, txs: Vec<O>) -> ExecutionPayload<O> {
        ExecutionPayload {
            parent_hash: self.parent_hash,
            fee_recipient: self.fee_recipient,
            state_root: self.state_root,
            receipts_root: self.receipts_root,
            logs_bloom: self.logs_bloom,
            prev_randao: self.prev_randao,
            block_number: self.block_number,
            gas_limit: self.gas_limit,
            gas_used: self.gas_used,
            timestamp: self.timestamp,
            extra_data: self.extra_data,
            base_fee_per_gas: self.base_fee_per_gas,
            block_hash: self.block_hash,
            transactions: txs,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NewPayloadError {
    #[error("{0}")]
    UnknownRpcError(RpcError),
    #[error(transparent)]
    JsonrpcQueryError(#[from] Error),
    #[error("{0}")]
    Custom(String),
}

impl From<RpcError> for NewPayloadError {
    fn from(value: RpcError) -> Self {
        Self::UnknownRpcError(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetPayloadError {
    #[error("Unknown payload")]
    UnknownPayload,
    #[error("{0}")]
    UnknownRpcError(RpcError),
    #[error(transparent)]
    JsonrpcQueryError(#[from] Error),
    #[error("{0}")]
    Custom(String),
}

impl From<RpcError> for GetPayloadError {
    fn from(value: RpcError) -> Self {
        match value.code {
            ErrorCode::ServerError(-38001) => Self::UnknownPayload,
            _ => Self::UnknownRpcError(value),
        }
    }
}
