use ethers_core::types::H256;
use fluct_core::{SequencerAPI, Transaction};
use tokio::sync::mpsc::UnboundedSender;

use crate::{ApiMessage, Error, Result};

#[derive(Clone)]
pub struct DevSequencerAPI {
    pub(crate) sender: UnboundedSender<ApiMessage>,
}

impl SequencerAPI for DevSequencerAPI {
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
}
