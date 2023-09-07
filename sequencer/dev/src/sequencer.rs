use std::io;

use crate::{ApiMessage, DevSequencerAPI, Error, Result};
use async_trait::async_trait;
use fluct_core::{SequencerService, Service, Transaction};
use fluct_service::{AsyncStepService, AsyncStepServiceWapper1};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

struct DevSequencer {
    receiver: UnboundedReceiver<ApiMessage>,
    sender: UnboundedSender<ApiMessage>,
    txpool: Vec<Transaction>,
}

impl Default for DevSequencer {
    fn default() -> Self {
        Self::new()
    }
}

impl DevSequencer {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded_channel();

        Self {
            receiver,
            sender,
            txpool: Vec::new(),
        }
    }

    async fn _step(&mut self) -> Result<()> {
        let msg = self.receiver.recv().await.ok_or(Error::ChannelClosed)?;

        match msg {
            ApiMessage::TxHash(hash) => {}
            ApiMessage::Transaction(tx) => self.txpool.push(tx),
        }

        Ok(())
    }
}

#[async_trait]
impl AsyncStepService for DevSequencer {
    type Error = Error;

    async fn step(&mut self) -> Result<()> {
        self._step().await
    }
}

pub struct DevSequencerService(AsyncStepServiceWapper1<DevSequencer>);

impl Default for DevSequencerService {
    fn default() -> Self {
        Self(AsyncStepServiceWapper1::new(DevSequencer::new()))
    }
}

impl Service for DevSequencerService {
    type Error = io::Error;

    fn start(&mut self) -> std::result::Result<(), Self::Error> {
        self.0.start()
    }

    fn stop(&mut self) -> std::result::Result<(), Self::Error> {
        self.0.stop()
    }

    fn kill(&mut self) -> std::result::Result<(), Self::Error> {
        self.0.kill()
    }
}

#[async_trait]
impl SequencerService for DevSequencerService {
    type API = DevSequencerAPI;

    fn api(&self) -> DevSequencerAPI {
        DevSequencerAPI {
            sender: self.0.service0().sender.clone(),
        }
    }

    fn txs(&self) -> &[Transaction] {
        &self.0.service0().txpool
    }
}
