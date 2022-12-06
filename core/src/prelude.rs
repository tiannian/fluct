use std::fmt::Debug;

pub trait VersionedKeyValueStore: Sized + Clone {
    type Error: Debug;

    fn open(name: &str) -> Result<Self, Self::Error>;

    fn vkv_set(&self, key: &[u8], value: Vec<u8>) -> Result<(), Self::Error>;

    fn vkv_del(&self, key: &[u8]) -> Result<(), Self::Error>;

    fn vkv_get(&self, key: &[u8], height: Option<u64>) -> Result<Vec<u8>, Self::Error>;

    fn latest_commit(&self) -> Result<u64, Self::Error>;

    fn commit(&self) -> Result<u64, Self::Error>;

    fn prune(&self, height: u64) -> Result<(), Self::Error>;
}
