use fluct_service::{CallError, StepError};
use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CallError(#[from] CallError),

    #[error(transparent)]
    OneShotRecvError(#[from] oneshot::error::RecvError),
}

impl StepError for Error {
    fn is_exit(&self) -> bool {
        match self {
            Self::CallError(CallError::ChannelClosed) => true,
            Self::OneShotRecvError(_) => false,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
