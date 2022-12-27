use alloc::vec::Vec;
use primitive_types::H256;

use crate::{Error, KeyValueStoreReadonly, StorageDiff};

pub struct StorageIter<KV: KeyValueStoreReadonly> {
    inner: KV::Range,
}

/* impl<KV: KeyValueStoreReadonly> StorageIter<KV> { */
    /* fn build(&mut self, n: Result<(Vec<u8>, Vec<u8>), KV::Error>) -> Result<H256, Error> { */
    /*     let (_, v) = n.map_err(Error::store)?; */
    /*  */
    /*     match StorageDiff::decode(&v)? { */
    /*         StorageDiff::DeleteAll => None, */
    /*     } */
    /*  */
    /*     Ok(H256::zero()) */
    /* } */
/* } */

impl<KV: KeyValueStoreReadonly> Iterator for StorageIter<KV> {
    type Item = Result<H256, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.inner.next()?;

        match n {
            Ok((_, v)) => {
                match StorageDiff::decode {
                    
                }
            }
            Err(e) => Some(Err(Error::store(e)))
        }
    }
}
