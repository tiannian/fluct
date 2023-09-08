use async_trait::async_trait;
use ethers_core::types::H256;
use fluct_core::{SequencerApi, Transaction};
use tokio::sync::mpsc::UnboundedSender;

use crate::{ApiMessage, Error, Result};

#[derive(Clone)]
pub struct DevSequencerApi {
    pub(crate) sender: UnboundedSender<ApiMessage>,
}

#[async_trait]
impl SequencerApi for DevSequencerApi {
    type Error = Error;

    fn broadcast_tx(&self, tx: Transaction) -> Result<()> {
        self.sender
            .send(ApiMessage::Transaction(tx))
            .map_err(|_| Error::ChannelClosed)?;
        Ok(())
    }

    fn comfirm_tx(&self, txhash: H256) -> Result<()> {
        self.sender
            .send(ApiMessage::TxHash(txhash))
            .map_err(|_| Error::ChannelClosed)?;
        Ok(())
    }

    async fn txs(&self) -> Result<Vec<Transaction>> {
        Ok(vec![])
    }
}
