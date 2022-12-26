/// Bytes for key-value store
pub type StoreBytes = Vec<u8>;

pub mod utils {
    pub fn u64_to_bytes(x: &u64) -> [u8; 8] {
        x.to_le_bytes()
    }

    pub fn u64_from_bytes(b: [u8; 8]) -> u64 {
        u64::from_le_bytes(b)
    }
}
