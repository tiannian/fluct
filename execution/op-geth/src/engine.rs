use async_trait::async_trait;
use ethers_core::types::Bytes;
use fluct_core::{
    transaction_utils, EngineApi, EngineError, ExecutionPayload, ForkChoiceResult, ForkChoiceState,
    PayloadAttributes, Status, Transaction,
};
use fluct_jsonrpc::client::{RpcClient, RpcResponse};
use serde::{Deserialize, Serialize};

use crate::Error;

/// Engine API of OpGeth
#[derive(Clone)]
pub struct GethEngineAPI {
    client: RpcClient,
}

impl GethEngineAPI {
    pub(crate) fn new(jwt: &[u8]) -> Result<Self, Error> {
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
} */
