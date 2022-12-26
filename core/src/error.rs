use core::fmt::Debug;

use alloc::string::String;

/// Error
#[derive(Debug)]
pub enum Error {
    RlpDecodeError(rlp::DecoderError),

    EthereumEnvelopedError(ethereum::EnvelopedDecoderError<rlp::DecoderError>),

    StoreError(String),
}

macro_rules! define_from_error {
    ($oe:ty, $te:ty, $ee:ident) => {
        impl From<$oe> for $te {
            fn from(e: $oe) -> Self {
                Self::$ee(e)
            }
        }
    };
}

define_from_error!(rlp::DecoderError, Error, RlpDecodeError);
define_from_error!(ethereum::EnvelopedDecoderError<rlp::DecoderError>, Error, EthereumEnvelopedError);

impl Error {
    pub fn store(e: impl Debug) -> Self {
        Self::StoreError(format!("{:?}", e))
    }
}

/// Result with Error
pub type Result<T> = core::result::Result<T, Error>;
