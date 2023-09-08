use fluct_service::StepError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Channel Closed")]
    ChannelClosed,
}

impl StepError for Error {
    fn is_exit(&self) -> bool {
        match self {
            Self::ChannelClosed => true,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
