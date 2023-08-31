use async_trait::async_trait;
use deop_core::{transaction_utils, types, ApiResult, EngineAPI, Transaction};
use ethers_core::types::{Block, BlockId, BlockNumber, Bytes};

pub struct Engine {
    url: String,
    jwt: String,
}

#[async_trait]
impl EngineAPI for Engine {
    async fn engine_fork_choice(
        &self,
        state: &types::ForkchoiceState,
        attr: &types::PayloadAttributesV1<Transaction>,
    ) -> ApiResult<types::ForkChoiceResult> {
        let txs = transaction_utils::transaction_to_bytes(attr.transactions);
    }

    async fn engine_new_payload(
        &self,
        payload: &types::ExecutionPayload<Transaction>,
    ) -> ApiResult<types::Status> {
    }

    async fn engine_get_payload(
        &self,
        payload_id: &Bytes,
    ) -> ApiResult<types::ExecutionPayload<Transaction>> {
    }

    async fn eth_block_number(&self) -> ApiResult<BlockNumber> {}

    async fn eth_chain_id(&self) -> ApiResult<u64> {}

    async fn eth_get_block(&self, block: BlockId) -> ApiResult<Block<Transaction>> {}
}
