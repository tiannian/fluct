use fluct_jsonrpc::{Error, ErrorCode, RpcError};
use thiserror::Error;

/// Engin Api error
#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Get error response")]
    EmptyResponse,
    #[error("Invalid forkchoice state")]
    InvalidForkChoiceState,
    #[error("Invalid payload attributes")]
    InvalidPayloadAttributes,
    #[error("{0}")]
    UnknownRpcError(RpcError),
    #[error(transparent)]
    JsonrpcQueryError(#[from] Error),
    #[error(transparent)]
    FluctCoreError(#[from] crate::Error),
    #[error("{0}")]
    Custom(String),
}

impl From<RpcError> for EngineError {
    fn from(value: RpcError) -> Self {
        match value.code {
            ErrorCode::ServerError(-38002) => Self::InvalidForkChoiceState,
            ErrorCode::ServerError(-38003) => Self::InvalidPayloadAttributes,
            _ => Self::UnknownRpcError(value),
        }
    }
}

/// Web3 Api error
#[derive(Debug, Error)]
pub enum Web3Error {
    #[error("Get error response")]
    EmptyResponse,

    #[error("{0}")]
    UnknownRpcError(RpcError),
    #[error(transparent)]
    JsonrpcQueryError(#[from] Error),
}

impl From<RpcError> for Web3Error {
    fn from(value: RpcError) -> Self {
        Self::UnknownRpcError(value)
    }
}
