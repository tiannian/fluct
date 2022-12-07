use crate::{Result, TransactionBytes};

pub struct Executor {}

impl Executor {
    pub fn new() -> Self {
        Executor {}
    }

    pub fn apply_block(txs: &[TransactionBytes]) -> Result<()> {
        Ok(())
    }
}
