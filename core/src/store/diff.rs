use alloc::vec::Vec;
use bytes::BytesMut;
use primitive_types::H256;

use crate::{Error, Result};

pub enum StorageDiff {
    DeleteAll,
    ModifySingle(Vec<H256>),
}

impl StorageDiff {
    pub fn encode(&self) -> BytesMut {
        let mut bytes = BytesMut::new();

        match self {
            Self::DeleteAll => bytes.extend_from_slice(&[1]),
            Self::ModifySingle(v) => {
                bytes.extend_from_slice(&[2]);
                bytes.extend_from_slice(&rlp::encode_list(&v));
            }
        }

        bytes
    }

    pub fn decode(data: &[u8]) -> Result<Self> {
        let first = data[0];

        match first {
            1 => Ok(Self::DeleteAll),
            2 => {
                let r = rlp::Rlp::new(&data[1..]);
                let v = r.as_list()?;
                Ok(Self::ModifySingle(v))
            }
            _ => Err(Error::WrongTypeId("StorageDiff")),
        }
    }
}
