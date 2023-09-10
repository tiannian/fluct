use fluct_core::Web3Error;
use fluct_service::{CallError, StepError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CallError(#[from] CallError),

    #[error("No web3 api configed")]
    NoWeb3ApiConfiged,

    #[error(transparent)]
    Web3Error(#[from] Web3Error),
}

impl StepError for Error {
    fn is_exit(&self) -> bool {
        match self {
            Self::CallError(CallError::ChannelClosed) => true,
            Self::CallError(CallError::SenderReject) => false,
            Self::NoWeb3ApiConfiged => true,
            Self::Web3Error(_) => true,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
