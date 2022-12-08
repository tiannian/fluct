use crate::{
    open_block_store_readonly, open_state_store_readonly, open_tx_store_readonly, BlockStore,
    KeyValueDb, Result, StateStore, TxStore,
};

pub struct Store<KV> {
    pub block: BlockStore<KV>,
    pub tx: TxStore<KV>,
    pub state: StateStore<KV>,
}

pub fn open_store_readonly<Db: KeyValueDb>(db: &Db) -> Result<Store<Db::KeyValueStoreReadonly>> {
    let block = open_block_store_readonly(db)?;
    let tx = open_tx_store_readonly(db)?;
    let state = open_state_store_readonly(db)?;

    Ok(Store { block, tx, state })
}
