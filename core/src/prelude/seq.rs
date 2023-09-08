use std::error::Error;

use async_trait::async_trait;
use ethers_core::types::H256;

use crate::{Service, Transaction};

/// Api of sequencer
///
/// Api can clone, It use to write mempool
#[async_trait]
pub trait SequencerApi: Clone {
    /// Error of sequencer
    type Error: Error + Sync + Send + 'static;

    /// Broadcast a transaction
    ///
    /// Put transaction into mempool
    fn broadcast_tx(&self, tx: Transaction) -> Result<(), Self::Error>;

    /// Comfirm a transaction
    ///
    /// Remove transaction from mempool
    fn comfirm_tx(&self, txhash: H256) -> Result<(), Self::Error>;

    /// Get transacion seqence.
    async fn txs(&self) -> Result<&[Transaction], Self::Error>;
}

/// Service of Sequencer, aka mempool(txpool) service
pub trait SequencerService: Service {
    /// Api of sequencer
    type Api: SequencerApi;

    /// Get Api instance
    fn api(&self) -> Self::Api;
}

/// Error Type of SequencerApi from SequencerService
pub type SequencerApiError<S> = <<S as SequencerService>::Api as SequencerApi>::Error;
