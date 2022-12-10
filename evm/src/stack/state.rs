use std::collections::{BTreeMap, BTreeSet};

use evm::executor::stack::{Log, MemoryStackAccount};
use primitive_types::{H160, H256};

#[derive(Debug, Clone, Default)]
pub struct State {
    pub logs: Vec<Log>,
    pub accounts: BTreeMap<H160, MemoryStackAccount>,
    pub storages: BTreeMap<(H160, H256), H256>,
    pub deletes: BTreeSet<H160>,
}
