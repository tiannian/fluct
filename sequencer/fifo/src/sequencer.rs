use std::io;

use crate::{ApiMessage, DevSequencerApi, Error, Result};
use async_trait::async_trait;
use fluct_core::{SequencerService, Service, Transaction, Web3Api};
use fluct_service::{AsyncStepService, AsyncStepServiceWapper1};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

struct DevSequencer {
    receiver: UnboundedReceiver<ApiMessage>,
    sender: UnboundedSender<ApiMessage>,
    txpool: Vec<Transaction>,
    web3_api: Option<Box<dyn Web3Api>>,
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
            web3_api: None,
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
}

#[async_trait]
impl SequencerService for DevSequencerService {
    type Api = DevSequencerApi;

    fn api(&self) -> DevSequencerApi {
        DevSequencerApi {
            sender: self.0.service0().sender.clone(),
        }
    }

    fn set_api(&mut self, web3_api: impl Web3Api) {
        self.0.service0_mut().web3_api = Some(Box::new(web3_api));
    }
}
