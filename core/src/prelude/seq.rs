use std::error::Error;

use async_trait::async_trait;

use crate::Transaction;

pub trait SequencerAPI: Clone {
    type Error: Error + Send + Sync + 'static;

    fn broadcast_tx(&self, tx: Transaction) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait Sequencer {
    type Error: Error + Send + Sync + 'static;

    type API: SequencerAPI;

    fn api(&self) -> Self::API;

    fn add_txs(&mut self, txs: Vec<Transaction>);

    fn claim_batch(&mut self) -> Vec<Transaction>;

    fn txs(&self) -> &[Transaction];

    /// Step Sequencer
    ///
    /// TODO: Maybe we need add StepedService for this service
    async fn step(&mut self) -> Result<(), Self::Error>;
}
