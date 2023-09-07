use std::error::Error;

use async_trait::async_trait;
use ethers_core::types::H256;

use crate::{Service, Transaction};

pub trait SequencerAPI: Clone {
    type Error: Error + Sync + Send + 'static;

    fn broadcast_tx(&self, tx: Transaction) -> Result<(), Self::Error>;

    fn comfirm_tx(&self, txhash: H256) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait SequencerService: Service {
    type API: SequencerAPI;

    fn api(&self) -> Self::API;

    fn txs(&self) -> &[Transaction];
}

pub type SequencerApiError<S> = <<S as SequencerService>::API as SequencerAPI>::Error;
