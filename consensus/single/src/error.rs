use ethers_core::types::H256;
use fluct_core::SequencerApi;
use fluct_service::CallError;
use thiserror::Error;

#[derive(Debug)]
pub enum Error<S>
where
    S: SequencerApi,
{
    SequencerApiError(S::Error),
}

pub type Result<T, S> = std::result::Result<T, Error<S>>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Verify Block Hash Failed, expect: {0}, got: {1}")]
    FailedVerifyBlockHash(H256, H256),

    #[error(transparent)]
    CallError(#[from] CallError),
}
