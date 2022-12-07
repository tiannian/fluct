use std::fmt::Debug;

use crate::StoreBytes;

pub trait VersionedKeyValueStore: KeyValueStore {
    fn vkv_set(&self, key: &[u8], value: Vec<u8>) -> Result<(), Self::Error>;

    fn vkv_del(&self, key: &[u8]) -> Result<(), Self::Error>;

    fn vkv_get(&self, key: &[u8], height: Option<u64>) -> Result<Vec<u8>, Self::Error>;

    fn latest(&self) -> Result<u64, Self::Error>;

    fn commit(&self) -> Result<u64, Self::Error>;

    fn prune(&self, height: u64) -> Result<(), Self::Error>;
}

pub trait KeyValueStore: KeyValueStoreReadonly {
    /// Do operation for set and del.
    fn ops(&self, ops: &[(impl AsRef<[u8]>, Option<StoreBytes>)]) -> Result<(), Self::Error>;
}

pub trait KeyValueStoreReadonly: Clone {
    type Error: Debug;

    /// Read value by key.
    fn get(&self, key: impl AsRef<[u8]>) -> Result<Option<StoreBytes>, Self::Error>;
}

pub trait KeyValueDb {
    type Error: Debug;

    type KeyValueStore: KeyValueStore;

    type KeyValueStoreReadonly: KeyValueStoreReadonly;

    fn open(&self, namespace: &str) -> Result<Self::KeyValueStore, Self::Error>;

    fn open_readonly(&self, namespace: &str) -> Result<Self::KeyValueStoreReadonly, Self::Error>;
}
