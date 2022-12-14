// pub struct Store {}

mod block;
pub use block::*;

mod tx;
pub use tx::*;

mod receipt;
pub use receipt::*;

mod state;
pub use state::*;

mod diff;
pub use diff::*;

use crate::{KeyValueDb, Result};

/// A store
pub struct Store<KV> {
    /// Block store
    pub block: BlockStore<KV>,
    /// Transaction store
    pub tx: TxStore<KV>,
    /// State store
    pub state: StateStore<KV>,
}

/// Open a readonly store
pub fn open_store_readonly<Db: KeyValueDb>(db: &Db) -> Result<Store<Db::KeyValueStoreReadonly>> {
    let block = open_block_store_readonly(db)?;
    let tx = open_tx_store_readonly(db)?;
    let state = open_state_store_readonly(db)?;

    Ok(Store { block, tx, state })
}

/// Open store
///
/// Used to write state when transaction execute.
pub fn open_store<Db: KeyValueDb>(db: &Db, height: u64) -> Result<Store<Db::KeyValueStore>> {
    let block = open_block_store(db)?;
    let tx = open_tx_store(db)?;
    let state = open_state_store(db, height)?;

    Ok(Store { block, tx, state })
}
