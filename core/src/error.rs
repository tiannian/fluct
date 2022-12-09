use std::fmt::Debug;

use thiserror::Error;

/// Error
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    RlpDecodeError(#[from] rlp::DecoderError),

    #[error("{0:?}")]
    EthereumEnvelopedError(ethereum::EnvelopedDecoderError<rlp::DecoderError>),

    #[error("{0}")]
    StoreError(String),
}

impl Error {
    pub fn store(e: impl Debug) -> Self {
        Self::StoreError(format!("{:?}", e))
    }
}

/// Result with Error
pub type Result<T> = std::result::Result<T, Error>;
