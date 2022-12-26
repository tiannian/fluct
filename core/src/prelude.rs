use core::fmt::Debug;

use crate::{utils, StoreBytes};

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

    fn set(&self, key: impl AsRef<[u8]>, value: StoreBytes) -> Result<(), Self::Error>;

    fn del(&self, key: impl AsRef<[u8]>) -> Result<(), Self::Error>;

    fn commit(&self) -> Result<(), Self::Error>;
}

/// Readonly key-value store.
pub trait KeyValueStoreReadonly: Clone {
    type Error: Debug;

    /// Read value by equal key.
    fn get(&self, key: impl AsRef<[u8]>) -> Result<Option<StoreBytes>, Self::Error>;

    type Range: Iterator<Item = Result<(StoreBytes, StoreBytes), Self::Error>>;

    /// Get range of [begin, end].
    ///
    /// Begin and end are closed.
    fn range(&self, begin: impl AsRef<[u8]>, end: impl AsRef<[u8]>, reverse: bool) -> Self::Range;
}

/// Versioned key-value store.
pub trait VersionedKeyValueReadOnly: KeyValueStoreReadonly {
    /// get value by versioned key
    fn get_by_version(
        &self,
        key: impl AsRef<[u8]>,
        version: u64,
    ) -> Result<Option<StoreBytes>, Self::Error> {
        let mut lt_key = key.as_ref().to_vec();
        lt_key.extend_from_slice(&utils::u64_to_bytes(&version));

        let mut iter = self.range(key, lt_key, true);

        if let Some(v) = iter.next() {
            let (_, mut v) = v?;
            if let Some(b) = v.first() {
                if *b == 1 {
                    Ok(Some(v.split_off(1)))
                } else {
                    Ok(None)
                }
            } else {
                Ok(Some(v))
            }
        } else {
            Ok(None)
        }
    }
}

impl<T: KeyValueStoreReadonly> VersionedKeyValueReadOnly for T {}

pub trait VersionedKeyValue: KeyValueStore {
    fn set_by_version(
        &self,
        key: impl AsRef<[u8]>,
        value: StoreBytes,
        version: u64,
    ) -> Result<(), Self::Error> {
        let mut key = key.as_ref().to_vec();
        key.extend_from_slice(&utils::u64_to_bytes(&version));

        let mut v = vec![1];
        v.extend(value);

        self.set(key, v)?;

        Ok(())
    }

    fn del_by_version(&self, key: impl AsRef<[u8]>, version: u64) -> Result<(), Self::Error> {
        let mut key = key.as_ref().to_vec();
        key.extend_from_slice(&utils::u64_to_bytes(&version));

        self.set(key, vec![0])?;

        Ok(())
    }
}

impl<T: KeyValueStore> VersionedKeyValue for T {}
