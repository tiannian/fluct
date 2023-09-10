use std::error::Error;

use async_trait::async_trait;

use crate::{
    types::{ConsensusGenesis, ForkChoiceState},
    Block, EngineApi, SequencerApi, Service, Transaction, Web3Api,
};

/// Service of Consensus
pub trait ConsensusService<SA>: Service + Sized
where
    SA: SequencerApi,
{
    /// Create node and init node using genesis, if node isn't inited
    fn init(
        &mut self,
        genesis: ConsensusGenesis<Transaction>,
        state: ForkChoiceState,
    ) -> Result<(), Self::Error>;

    fn set_api(&mut self, engine_api: impl EngineApi, web3_api: impl Web3Api, seqencer_api: SA);
}

#[async_trait]
pub trait ConsensusApi {
    type Error: Error;

    async fn add_block(&mut self, block: Block) -> Result<(), Self::Error>;

    /// Get chain state
    async fn chain_state(&self) -> Result<ForkChoiceState, Self::Error>;
}
