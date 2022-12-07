use ethereum_types::{H160, H256, U256};

use crate::{Error, KeyValueDb, KeyValueStoreReadonly, Result, VersionedKeyValueReadOnly};

pub struct StateStore<KV> {
    /// address -> (balance, nonce, code)
    pub(crate) account: KV,

    pub(crate) code: KV,

    /// Versioned storage address -> index -> bytes
    pub(crate) storage: KV,
}

pub fn open_state_store_readonly<Db: KeyValueDb>(
    db: Db,
) -> Result<StateStore<Db::KeyValueStoreReadonly>> {
    let account = db.open_readonly("state_account").map_err(Error::store)?;
    let storage = db.open_readonly("state_storage").map_err(Error::store)?;
    let code = db.open_readonly("state_code").map_err(Error::store)?;

    Ok(StateStore {
        account,
        storage,
        code,
    })
}

pub fn open_state_store<Db: KeyValueDb>(db: Db) -> Result<StateStore<Db::KeyValueStore>> {
    let account = db.open("state_account").map_err(Error::store)?;
    let storage = db.open("state_storage").map_err(Error::store)?;
    let code = db.open("state_code").map_err(Error::store)?;

    Ok(StateStore {
        account,
        storage,
        code,
    })
}

impl<KV: KeyValueStoreReadonly> StateStore<KV> {
    pub fn get_basic(&self, address: H160, height: u64) -> Result<Option<(U256, U256)>> {
        let account = &self.account;

        if let Some(data) = account
            .get_by_version(address.as_bytes(), height)
            .map_err(Error::store)?
        {
            let balance = U256::from_little_endian(&data);
            let nonce = U256::from_little_endian(&data[32..]);

            Ok(Some((balance, nonce)))
        } else {
            Ok(None)
        }
    }

    pub fn get_code(&self, address: H160, height: u64) -> Result<Option<Vec<u8>>> {
        self.code
            .get_by_version(address.as_bytes(), height)
            .map_err(Error::store)
    }

    pub fn get_storage(&self, address: H160, index: H256, height: u64) -> Result<Option<H256>> {
        let mut key: Vec<u8> = address.0.into();
        key.extend_from_slice(index.as_bytes());
        if let Some(data) = self
            .storage
            .get_by_version(key, height)
            .map_err(Error::store)?
        {
            Ok(Some(H256::from_slice(&data)))
        } else {
            Ok(None)
        }
    }
}
