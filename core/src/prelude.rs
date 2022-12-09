use std::fmt::Debug;

use crate::StoreBytes;

/// Database for key value.
pub trait KeyValueDb {
    /// Error of key vlaue store.
    type Error: Debug;

    /// Writable key-value store.
    type KeyValueStore: KeyValueStore;

    /// Readonly key-value store.
    type KeyValueStoreReadonly: KeyValueStoreReadonly;

    /// Open writable key-value store.
    fn open(&self, namespace: &str) -> Result<Self::KeyValueStore, Self::Error>;

    /// Open readonly key-value store.
    fn open_readonly(&self, namespace: &str) -> Result<Self::KeyValueStoreReadonly, Self::Error>;
}

/// Writable key-value store.
pub trait KeyValueStore: KeyValueStoreReadonly {
    /// Do operation for set and del.
    fn ops(&self, ops: &[(impl AsRef<[u8]>, Option<StoreBytes>)]) -> Result<(), Self::Error>;
}

/// Readonly key-value store.
pub trait KeyValueStoreReadonly: Clone {
    type Error: Debug;

    /// Read value by equal key.
    fn get(&self, key: impl AsRef<[u8]>) -> Result<Option<StoreBytes>, Self::Error>;

    /// Read value by less key.
    fn get_lt_prefix(
        &self,
        prefix: impl AsRef<[u8]>,
        key: impl AsRef<[u8]>,
    ) -> Result<Option<StoreBytes>, Self::Error>;
}

/// Versioned key-value store.
pub trait VersionedKeyValueReadOnly: KeyValueStoreReadonly {
    /// get value by versioned key
    fn get_by_version(
        &self,
        key: impl AsRef<[u8]>,
        version: u64,
    ) -> Result<Option<StoreBytes>, Self::Error> {
        if let Some(data) = self.get(&key)? {
            Ok(Some(data))
        } else {
            let mut lt_key = key.as_ref().to_vec();
            lt_key.extend_from_slice(&version.to_le_bytes());

            self.get_lt_prefix(key, lt_key)
        }
    }
}

impl<T: KeyValueStoreReadonly> VersionedKeyValueReadOnly for T {}
