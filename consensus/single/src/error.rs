use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use ethers_core::types::H256;
use fluct_service::{CallError, StepError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    SequencerApiError(String),
}

impl StepError for Error {
    fn is_exit(&self) -> bool {
        match self {
            Self::SequencerApiError(_) => false,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Verify Block Hash Failed, expect: {0}, got: {1}")]
    FailedVerifyBlockHash(H256, H256),

    #[error(transparent)]
    CallError(#[from] CallError),
}
