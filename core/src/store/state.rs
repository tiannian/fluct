use alloc::{collections::BTreeMap, vec::Vec};
use primitive_types::{H160, H256, U256};

use crate::{
    Error, KeyValueDb, KeyValueStore, KeyValueStoreReadonly, Result, StorageDiff,
    VersionedKeyValue, VersionedKeyValueReadOnly,
};

pub(crate) struct Writeable {
    pub height: u64,
    pub diffs: BTreeMap<H160, StorageDiff>,
}

impl Writeable {
    pub fn new(height: u64) -> Self {
        Self {
            height,
            diffs: BTreeMap::new(),
        }
    }
}

/// State store
///
/// Include account basic (balance, nonce), code, and storage.
pub struct StateStore<KV> {
    /// address -> (balance, nonce)
    pub(crate) account: KV,

    /// address -> code
    pub(crate) code: KV,

    /// Versioned storage address -> index -> bytes
    pub(crate) storage: KV,

    /// Versioned address's index
    ///
    /// address -> StorageDiff
    pub(crate) storage_index: KV,

    pub(crate) writeable: Option<Writeable>,
}

/// Open a readonly state store
pub fn open_state_store_readonly<Db: KeyValueDb>(
    db: &Db,
) -> Result<StateStore<Db::KeyValueStoreReadonly>> {
    let account = db.open_readonly("state_account").map_err(Error::store)?;
    let storage = db.open_readonly("state_storage").map_err(Error::store)?;
    let code = db.open_readonly("state_code").map_err(Error::store)?;
    let storage_index = db
        .open_readonly("state_storage_index")
        .map_err(Error::store)?;

    Ok(StateStore {
        account,
        storage,
        code,
        storage_index,
        writeable: None,
    })
}

/// Open a writeable state store
pub fn open_state_store<Db: KeyValueDb>(
    db: &Db,
    height: u64,
) -> Result<StateStore<Db::KeyValueStore>> {
    let account = db.open("state_account").map_err(Error::store)?;
    let storage = db.open("state_storage").map_err(Error::store)?;
    let code = db.open("state_code").map_err(Error::store)?;
    let storage_index = db.open("state_storage_index").map_err(Error::store)?;

    Ok(StateStore {
        account,
        storage,
        code,
        storage_index,
        writeable: Some(Writeable::new(height)),
    })
}

impl<KV: KeyValueStoreReadonly> StateStore<KV> {
    /// Get basic of account basic. Include balance, nonce.
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

    /// Get account's code
    pub fn get_code(&self, address: H160, height: u64) -> Result<Option<Vec<u8>>> {
        self.code
            .get_by_version(address.as_bytes(), height)
            .map_err(Error::store)
    }

    /// Get account's storage
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

impl<KV: KeyValueStore> StateStore<KV> {
    pub fn height(&self) -> Result<u64> {
        Ok(self
            .writeable
            .as_ref()
            .ok_or(Error::WrongInitalForWriteableState)?
            .height)
    }

    fn writeable_mut(&mut self) -> Result<&mut Writeable> {
        self.writeable
            .as_mut()
            .ok_or(Error::WrongInitalForWriteableState)
    }

    fn set_modify_index(&mut self, addr: H160, index: H256) -> Result<()> {
        let diffs = &mut self.writeable_mut()?.diffs;

        if let Some(StorageDiff::ModifySingle(v)) = diffs.get_mut(&addr) {
            v.push(index)
        } else {
            diffs.insert(addr, StorageDiff::ModifySingle(vec![index]));
        }
        Ok(())
    }

    pub fn set_basic(&self, addr: H160, balance: U256, nonce: U256) -> Result<()> {
        let mut data = vec![0u8; 64];
        balance.to_little_endian(&mut data);
        nonce.to_little_endian(&mut data[32..]);

        self.account
            .set_by_version(addr.as_bytes(), data, self.height()?)
            .map_err(Error::store)?;

        Ok(())
    }

    pub fn del_basic(&self, addr: H160) -> Result<()> {
        self.account
            .del_by_version(addr.as_bytes(), self.height()?)
            .map_err(Error::store)?;
        Ok(())
    }

    pub fn set_code(&self, addr: H160, code: Vec<u8>) -> Result<()> {
        self.code
            .set_by_version(addr.as_bytes(), code, self.height()?)
            .map_err(Error::store)?;
        Ok(())
    }

    pub fn del_code(&self, addr: H160) -> Result<()> {
        self.code
            .del_by_version(addr.as_bytes(), self.height()?)
            .map_err(Error::store)?;
        Ok(())
    }

    pub fn set_storage(&mut self, addr: H160, index: H256, value: H256) -> Result<()> {
        let mut key: Vec<u8> = addr.0.into();
        key.extend_from_slice(index.as_bytes());

        self.storage
            .set_by_version(key, value.0.into(), self.height()?)
            .map_err(Error::store)?;

        self.set_modify_index(addr, index)?;

        Ok(())
    }

    pub fn del_storage(&mut self, addr: H160, index: H256) -> Result<()> {
        let mut key: Vec<u8> = addr.0.into();
        key.extend_from_slice(index.as_bytes());

        self.storage
            .del_by_version(key, self.height()?)
            .map_err(Error::store)?;

        self.set_modify_index(addr, index)?;

        Ok(())
    }

    pub fn del_all_storage(&mut self, addr: H160) -> Result<()> {
        self.writeable_mut()?
            .diffs
            .insert(addr, StorageDiff::DeleteAll);

        // Delete all keys

        Ok(())
    }
}
