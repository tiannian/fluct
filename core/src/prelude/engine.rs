use async_trait::async_trait;
use ethers_core::types::{Block, BlockId, BlockNumber, Bytes};
use fluct_jsonrpc::Result as RpcResult;
use serde::{Deserialize, Serialize};

use crate::{types, Service, Transaction};

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
        attr: &types::PayloadAttributes<Transaction>,
    ) -> Result<types::ForkChoiceResult, types::EngineError>;

    async fn engine_new_payload(
        &self,
        payload: &types::ExecutionPayload<Transaction>,
    ) -> RpcResult<types::Status>;

    async fn engine_get_payload(
        &self,
        payload_id: &Bytes,
    ) -> RpcResult<types::ExecutionPayload<Transaction>>;

    async fn eth_block_number(&self) -> RpcResult<BlockNumber>;

    async fn eth_chain_id(&self) -> RpcResult<u64>;

    async fn eth_get_block(&self, block: BlockId) -> RpcResult<Block<Transaction>>;
}
