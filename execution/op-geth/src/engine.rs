use async_trait::async_trait;
use ethers_core::types::{BlockNumber, Bytes, H256};
use fluct_core::{
    transaction_utils, EngineApi, EngineError, ExecutionPayload, ForkChoiceResult, ForkChoiceState,
    PayloadAttributes, Status, Transaction,
};
use fluct_jsonrpc::client::{RpcClient, RpcResponse};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Clone)]
pub struct GethEngineAPI {
    client: RpcClient,
}

impl GethEngineAPI {
    pub fn new(jwt: &[u8]) -> Result<Self, Error> {
        let client = RpcClient::new("http://127.0.0.1:8551", Some(jwt))?;
        Ok(Self { client })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
enum EngineCall {
    #[serde(rename = "engine_forkchoiceUpdatedV1")]
    ForkChoice(ForkChoiceState, PayloadAttributes<Bytes>),
    #[serde(rename = "engine_newPayloadV1")]
    NewPayload(ExecutionPayload<Bytes>),
    #[serde(rename = "engine_getPayloadV1")]
    GetPayload((Bytes,)),
    #[serde(rename = "eth_chainId")]
    ChainId(()),
    #[serde(rename = "eth_blockNumber")]
    BlockNumber(()),
    #[serde(rename = "eth_getBlockByNumber")]
    GetBlockByNumber((BlockNumber, bool)),
    #[serde(rename = "eth_getBlockByHash")]
    GetBlockByHash((H256, bool)),
    #[serde(rename = "eth_syncing")]
    Syncing(()),
}

#[async_trait]
impl EngineApi for GethEngineAPI {
    async fn engine_fork_choice(
        &mut self,
        state: ForkChoiceState,
        attr: PayloadAttributes<Transaction>,
    ) -> Result<ForkChoiceResult, EngineError> {
        let txs = transaction_utils::transaction_to_bytes(&attr.transactions);

        let req = EngineCall::ForkChoice(state, attr.into_other_tx(txs));

        let res: RpcResponse<ForkChoiceResult> = self.client.call(req).await?;
        let res = res.into_result()?;
        let res = res.ok_or(EngineError::EmptyResponse)?;

        Ok(res)
    }

    async fn engine_new_payload(
        &mut self,
        payload: ExecutionPayload<Transaction>,
    ) -> Result<Status, EngineError> {
        let txs = transaction_utils::transaction_to_bytes(&payload.transactions);

        let req = EngineCall::NewPayload(payload.into_other_tx(txs));

        let res: RpcResponse<Status> = self.client.call(req).await?;
        let res = res.into_result()?;
        let res = res.ok_or(EngineError::EmptyResponse)?;

        Ok(res)
    }

    async fn engine_get_payload(
        &mut self,
        payload_id: Bytes,
    ) -> Result<ExecutionPayload<Transaction>, EngineError> {
        let req = EngineCall::GetPayload((payload_id,));

        let res: RpcResponse<ExecutionPayload<Bytes>> = self.client.call(req).await?;
        let res = res.into_result()?;
        let res = res.ok_or(EngineError::EmptyResponse)?;

        let txs = transaction_utils::bytes_to_transaction(&res.transactions)?;
        let res = res.into_other_tx(txs);
        Ok(res)
    }
}

/* impl GethEngineAPI {
    async fn eth_block_number(&mut self) -> Result<u64, types::Web3Error> {
        let req = EngineCall::BlockNumber(());

        let res: RpcResponse<U64> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(types::Web3Error::EmptyResponse)?;

        Ok(res.as_u64())
    }

    async fn eth_chain_id(&mut self) -> Result<u64, types::Web3Error> {
        let req = EngineCall::ChainId(());

        let res: RpcResponse<U64> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(types::Web3Error::EmptyResponse)?;

        Ok(res.as_u64())
    }

    async fn eth_get_block(
        &mut self,
        block: BlockId,
    ) -> Result<Block<Transaction>, types::Web3Error> {
        let req = match block {
            BlockId::Hash(v) => EngineCall::GetBlockByHash((v, true)),
            BlockId::Number(v) => EngineCall::GetBlockByNumber((v, true)),
        };

        let res: RpcResponse<Block<Transaction>> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(types::Web3Error::EmptyResponse)?;

        Ok(res)
    }

    async fn eth_syncing(&mut self) -> Result<SyncingStatus, types::Web3Error> {
        let req = EngineCall::Syncing(());

        let res: RpcResponse<SyncingStatus> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(types::Web3Error::EmptyResponse)?;

        Ok(res)
    }
} */
