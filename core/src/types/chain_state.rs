use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ChainState {
    pub latest: u64,
    pub safe: u64,
    pub finalized: u64,
}
