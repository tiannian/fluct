use evm::backend::Backend;
use fluct_core::BlockStore;

pub struct CoreBackend<KV> {
    block: BlockStore<KV>,
}

impl<KV> Backend for CoreBackend<KV> {}
