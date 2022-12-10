use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing field to build runtime")]
    MissingFieldToBuildRuntime,
}

pub type Result<T> = std::result::Result<T, Error>;
