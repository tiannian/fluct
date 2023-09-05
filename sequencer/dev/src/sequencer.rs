use std::mem;

use async_trait::async_trait;
use fluct_core::{Sequencer, Transaction};
use tokio::sync::mpsc::{
    error::TryRecvError, unbounded_channel, UnboundedReceiver, UnboundedSender,
};

use crate::{DevSequencerAPI, Error, Result};

pub struct DevSequencer {
    receiver: UnboundedReceiver<Transaction>,
    sender: UnboundedSender<Transaction>,
    txpool: Vec<Transaction>,
    pub max_num_tx: usize,
}

impl DevSequencer {
    pub fn new(max_num_tx: usize) -> Self {
        let (sender, receiver) = unbounded_channel();

        Self {
            receiver,
            sender,
            txpool: Vec::new(),
            max_num_tx,
        }
    }
}

#[async_trait]
impl Sequencer for DevSequencer {
    type Error = Error;

    type API = DevSequencerAPI;

    fn api(&self) -> Self::API {
        Self::API {
            sender: self.sender.clone(),
        }
    }

    fn add_txs(&mut self, txs: Vec<Transaction>) {
        self.txpool.extend_from_slice(&txs);
    }

    fn claim_batch(&mut self) -> Vec<Transaction> {
        mem::take(&mut self.txpool)
    }

    fn txs(&self) -> &[Transaction] {
        &self.txpool
    }

    async fn step(&mut self) -> Result<()> {
        for _ in 0..self.max_num_tx {
            match self.receiver.try_recv() {
                Ok(tx) => self.txpool.push(tx),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Err(Error::ChannelClosed),
            }
        }

        Ok(())
    }
}
