use std::fmt::Debug;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    RlpDecodeError(#[from] rlp::DecoderError),

    #[error(transparent)]
    EthereumEnvelopedError(#[from] ethereum::EnvelopedDecoderError<rlp::DecoderError>),

    #[error("{0}")]
    StoreError(String),
}

impl Error {
    pub fn store(e: impl Debug) -> Self {
        Self::StoreError(format!("{:?}", e))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
