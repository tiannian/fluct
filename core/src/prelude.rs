use std::fmt::Debug;

use async_trait::async_trait;
use ethers_core::types::{Block, BlockId, BlockNumber, Bytes};

use crate::{types, EngineResult};

pub trait Service {
    type Error: Debug;

    fn start(&self) -> Result<(), Self::Error>;

    fn stop(&self) -> Result<(), Self::Error>;

    fn kill(&self) -> Result<(), Self::Error>;
}

pub trait ExecutionService {
    type EngineAPI: EngineAPI;

    fn engine(&self) -> Self::EngineAPI;
}

#[async_trait]
pub trait EngineAPI {
    type Transaction;

    async fn engine_fork_choice(
        &self,
        state: &types::ForkchoiceState,
        attr: &types::PayloadAttributesV1<Self::Transaction>,
    ) -> EngineResult<types::ForkChoiceResult>;

    async fn engine_new_payload(
        &self,
        payload: &types::ExecutionPayload<Self::Transaction>,
    ) -> EngineResult<types::Status>;

    async fn engine_get_payload(
        &self,
        payload_id: &Bytes,
    ) -> EngineResult<types::ExecutionPayload<Self::Transaction>>;

    async fn eth_block_number(&self) -> EngineResult<BlockNumber>;

    async fn eth_chain_id(&self) -> EngineResult<u64>;

    async fn eth_get_block(&self, block: BlockId) -> EngineResult<Block<Self::Transaction>>;
}
