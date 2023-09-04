use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No Geth Binary Found")]
    NoGethBinaryFound,

    #[error("Execute Error: {0}")]
    SubprocessExecuteError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SubprocessError(#[from] subprocess::PopenError),
}

pub type Result<T> = std::result::Result<T, Error>;
