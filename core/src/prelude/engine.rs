use async_trait::async_trait;
use ethers_core::types::{Block, BlockId, Bytes, SyncingStatus};
use serde::{Deserialize, Serialize};

use crate::{types, Service, Transaction};

pub trait ExecutionService: Service {
    type API: EngineAPI;

    type Genesis: Serialize + for<'de> Deserialize<'de>;

    fn api(&self) -> Result<Self::API, Self::Error>;

    fn init(&mut self, genesis: Self::Genesis) -> Result<(), Self::Error>;

    /// Remove all data in engine.
    ///
    /// This method only can call in devnode.
    fn reset(&mut self) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait EngineAPI: Clone {
    async fn engine_fork_choice(
        &mut self,
        state: types::ForkChoiceState,
        attr: types::PayloadAttributes<Transaction>,
    ) -> Result<types::ForkChoiceResult, types::EngineError>;

    async fn engine_new_payload(
        &mut self,
        payload: types::ExecutionPayload<Transaction>,
    ) -> Result<types::Status, types::EngineError>;

    async fn engine_get_payload(
        &mut self,
        payload_id: Bytes,
    ) -> Result<types::ExecutionPayload<Transaction>, types::EngineError>;

    async fn eth_block_number(&mut self) -> Result<u64, types::Web3Error>;

    async fn eth_chain_id(&mut self) -> Result<u64, types::Web3Error>;

    async fn eth_get_block(
        &mut self,
        block: BlockId,
    ) -> Result<Block<Transaction>, types::Web3Error>;

    async fn eth_syncing(&mut self) -> Result<SyncingStatus, types::Web3Error>;
}
