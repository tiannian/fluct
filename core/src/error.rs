use ethers_core::utils::rlp::DecoderError;
use thiserror::Error;

/// Error
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    RlpDecodeError(#[from] DecoderError),
}
