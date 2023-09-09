use thiserror::Error;

/// OpGeth Error
#[derive(Debug, Error)]
pub enum Error {
    #[error("No Geth Binary Found")]
    NoGethBinaryFound,

    #[error("Execute Error: {0}")]
    SubprocessExecuteError(String),

    #[error("Failed to convert path to utf8")]
    FailedToGetUTF8Path,

    #[error("No instance start")]
    NoInstanceStart,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SubprocessError(#[from] subprocess::PopenError),

    #[error(transparent)]
    JsonRPCError(#[from] fluct_jsonrpc::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

/// Result alias of OpGeth Error
pub type Result<T> = std::result::Result<T, Error>;
