use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Channel Closed")]
    ChannelClosed,
}

pub type Result<T> = std::result::Result<T, Error>;
