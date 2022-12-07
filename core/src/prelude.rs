use std::fmt::Debug;

use crate::StoreBytes;

pub trait KeyValueDb {
    type Error: Debug;

    type KeyValueStore: KeyValueStore;

    type KeyValueStoreReadonly: KeyValueStoreReadonly;

    fn open(&self, namespace: &str) -> Result<Self::KeyValueStore, Self::Error>;

    fn open_readonly(&self, namespace: &str) -> Result<Self::KeyValueStoreReadonly, Self::Error>;
}

pub trait KeyValueStore: KeyValueStoreReadonly {
    /// Do operation for set and del.
    fn ops(&self, ops: &[(impl AsRef<[u8]>, Option<StoreBytes>)]) -> Result<(), Self::Error>;
}

pub trait KeyValueStoreReadonly: Clone {
    type Error: Debug;

    /// Read value by key.
    fn get(&self, key: impl AsRef<[u8]>) -> Result<Option<StoreBytes>, Self::Error>;

    fn get_lt_prefix(
        &self,
        prefix: impl AsRef<[u8]>,
        key: impl AsRef<[u8]>,
    ) -> Result<Option<StoreBytes>, Self::Error>;
}

pub trait VersionedKeyValueReadOnly: KeyValueStoreReadonly {
    fn get_by_version(
        &self,
        key: impl AsRef<[u8]>,
        version: u64,
    ) -> Result<Option<StoreBytes>, Self::Error> {
        let mut lt_key = key.as_ref().to_vec();
        lt_key.extend_from_slice(&version.to_le_bytes());

        if let Some(data) = self.get_lt_prefix(key, lt_key)? {
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }
}

impl<T: KeyValueStoreReadonly> VersionedKeyValueReadOnly for T {}
