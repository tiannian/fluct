use ethers_core::types::H256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ChainState {
    pub latest: H256,
    pub safe: H256,
    pub finalized: H256,
}
