use std::error::Error;

use async_trait::async_trait;

use crate::{
    types::{ConsensusGenesis, ForkChoiceState},
    EngineApi, SequencerApi, Service, Transaction, Web3Api,
};

/// Service of Consensus
pub trait ConsensusService: Service + Sized {
    /// Create node and init node using genesis, if node isn't inited
    fn init(
        &mut self,
        genesis: ConsensusGenesis<Transaction>,
        state: ForkChoiceState,
    ) -> Result<Self, Self::Error>;

    fn set_api(
        &mut self,
        engine_api: impl EngineApi,
        web3_api: impl Web3Api,
        seqencer_api: impl SequencerApi,
    );
}

#[async_trait]
pub trait ConsensusApi {
    type Error: Error;

    async fn add_block(&mut self) -> Result<(), Self::Error>;

    /// Get chain state
    async fn chain_state(&self) -> Result<ForkChoiceState, Self::Error>;
}
