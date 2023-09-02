use fluct_jsonrpc::{Error, ErrorCode, RpcError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
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

impl From<RpcError> for EngineError {
    fn from(value: RpcError) -> Self {
        match value.code {
            ErrorCode::ServerError(-38002) => Self::InvalidForkChoiceState,
            ErrorCode::ServerError(-38003) => Self::InvalidPayloadAttributes,
            _ => Self::UnknownRpcError(value),
        }
    }
}
