use fluct_core::{SequencerAPI, Transaction};
use tokio::sync::mpsc::UnboundedSender;

use crate::{Error, Result};

#[derive(Clone)]
pub struct DevSequencerAPI {
    pub(crate) sender: UnboundedSender<Transaction>,
}

impl SequencerAPI for DevSequencerAPI {
    type Error = Error;

    fn broadcast_tx(&self, tx: Transaction) -> Result<()> {
        self.sender.send(tx).map_err(|_| Error::ChannelClosed)?;
        Ok(())
    }
}
