use async_trait::async_trait;
use ethers_core::types::{Block, BlockId, BlockNumber, Bytes};
use serde::{Deserialize, Serialize};

use crate::{types, ApiResult, Service, Transaction};

pub trait ExecutionService: Service {
    type API: EngineAPI;

    type Genesis: Serialize + for<'de> Deserialize<'de>;

    fn api(&self) -> Self::API;

    fn init(&mut self, genesis: &Self::Genesis) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait EngineAPI {
    async fn engine_fork_choice(
        &self,
        state: &types::ForkchoiceState,
        attr: &types::PayloadAttributesV1<Transaction>,
    ) -> ApiResult<types::ForkChoiceResult>;

    async fn engine_new_payload(
        &self,
        payload: &types::ExecutionPayload<Transaction>,
    ) -> ApiResult<types::Status>;

    async fn engine_get_payload(
        &self,
        payload_id: &Bytes,
    ) -> ApiResult<types::ExecutionPayload<Transaction>>;

    async fn eth_block_number(&self) -> ApiResult<BlockNumber>;

    async fn eth_chain_id(&self) -> ApiResult<u64>;

    async fn eth_get_block(&self, block: BlockId) -> ApiResult<Block<Transaction>>;
}
