use std::{collections::HashMap, io};

use crate::{ApiRequest, ApiResponse, DevSequencerApi, Error, Result};
use async_trait::async_trait;
use ethers_core::types::H256;
use fluct_core::{SequencerService, Service, Transaction, Web3Api};
use fluct_service::{
    local_rpc, AsyncStepService, AsyncStepServiceWapper1, CallError, Caller, Hander,
};

struct DevSequencer {
    handler: Hander<ApiRequest, ApiResponse>,
    caller: Caller<ApiRequest, ApiResponse>,
    txpool: Vec<Transaction>,
    txindexer: HashMap<H256, usize>,
    web3_api: Option<Box<dyn Web3Api>>,
}

impl Default for DevSequencer {
    fn default() -> Self {
        Self::new()
    }
}

impl DevSequencer {
    pub fn new() -> Self {
        let (handler, caller) = local_rpc();

        Self {
            handler,
            caller,
            txpool: Vec::new(),
            txindexer: HashMap::new(),
            web3_api: None,
        }
    }

    async fn _step(&mut self) -> Result<()> {
        let (rep, resper) = self.handler.recv().await?;

        match rep {
            ApiRequest::Transaction(tx) => {
                self.txindexer.insert(tx.hash, self.txpool.len());
                self.txpool.push(tx);
            }
            ApiRequest::TxHash(txhash) => {
                if let Some(index) = self.txindexer.get(&txhash) {
                    self.txpool.remove(*index);
                }
            }
            ApiRequest::GetAllTransaction => {
                if let Some(resper) = resper {
                    resper
                        .send(ApiResponse::GetAllTransaction(self.txpool.clone()))
                        .map_err(|_| CallError::ChannelClosed)?;
                } else {
                    log::warn!("Use send method to get txpool")
                }
            }
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
            caller: self.0.service0().caller.clone(),
        }
    }

    fn set_api(&mut self, web3_api: impl Web3Api) {
        self.0.service0_mut().web3_api = Some(Box::new(web3_api));
    }
}
