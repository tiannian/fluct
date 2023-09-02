use async_trait::async_trait;
use ethers_core::types::{Block, BlockId, BlockNumber, Bytes};
use fluct_core::{
    transaction_utils,
    types::{self, PayloadAttributes},
    EngineAPI, Transaction,
};
use fluct_jsonrpc::{
    client::{RpcClient, RpcResponse},
    Error, RpcError,
};
use serde::{Deserialize, Serialize};

use crate::OpGethParser;

pub struct Engine {
    client: RpcClient,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
enum EngineCall {
    #[serde(rename = "engine_forkchoiceUpdatedV1")]
    ForkChoice(types::ForkchoiceState, types::PayloadAttributes<Bytes>),
    #[serde(rename = "engine_newPayloadV1")]
    NewPayload(types::ExecutionPayload<Bytes>),
    #[serde(rename = "engine_getPayloadV1")]
    GetPayload((Bytes,)),
}

impl Engine {
    async fn engine_fork_choice(
        &mut self,
        state: types::ForkchoiceState,
        attr: types::PayloadAttributes<Transaction>,
    ) -> Result<types::ForkChoiceResult, types::ForkChoiceError> {
        let txs = transaction_utils::transaction_to_bytes::<OpGethParser>(&attr.transactions)
            .map_err(|e| types::ForkChoiceError::Custom(format!("{e}")))?;

        let req = EngineCall::ForkChoice(state, attr.into_other_tx(txs));

        let res: RpcResponse<types::ForkChoiceResult> = self.client.call(req).await?;
        let res = res.into_result()?;
        let res = res.ok_or(Error::Custom("Failed to get return value".to_string()))?;

        Ok(res)
    }

    async fn engine_new_payload(
        &mut self,
        payload: types::ExecutionPayload<Transaction>,
    ) -> Result<types::Status, types::NewPayloadError> {
        let txs = transaction_utils::transaction_to_bytes::<OpGethParser>(&payload.transactions)
            .map_err(|e| types::NewPayloadError::Custom(format!("{e}")))?;

        let req = EngineCall::NewPayload(payload.into_other_tx(txs));

        let res: RpcResponse<types::Status> = self.client.call(req).await?;
        let res = res.into_result()?;
        let res = res.ok_or(Error::Custom("Failed to get return value".to_string()))?;

        Ok(res)
    }

    async fn engine_get_payload(
        &mut self,
        payload_id: Bytes,
    ) -> Result<types::ExecutionPayload<Transaction>, types::GetPayloadError> {
        let req = EngineCall::GetPayload((payload_id,));

        let res: RpcResponse<types::ExecutionPayload<Bytes>> = self.client.call(req).await?;
        let res = res.into_result()?;
        let res = res.ok_or(Error::Custom("Failed to get return value".to_string()))?;

        let txs = transaction_utils::bytes_to_transaction::<OpGethParser>(&res.transactions)
            .map_err(|e| types::GetPayloadError::Custom(format!("{e}")))?;
        let res = res.into_other_tx(txs);
        Ok(res)
    }

    /*
    async fn eth_block_number(&self) -> Result<BlockNumber> {}

    async fn eth_chain_id(&self) -> Result<u64> {}

    async fn eth_get_block(&self, block: BlockId) -> Result<Block<Transaction>> {} */
}
