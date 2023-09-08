use ethers_core::types::{BlockNumber, H256, U64};
use fluct_core::{Web3Error, Web3Result};
use fluct_jsonrpc::client::{RpcClient, RpcResponse};
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Clone)]
pub struct GethWeb3Api {
    client: RpcClient,
}

impl GethWeb3Api {
    pub fn new() -> Result<Self> {
        let client = RpcClient::new("http://127.0.0.1:8545", None)?;
        Ok(Self { client })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
enum Web3Call {
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

impl GethWeb3Api {
    async fn eth_block_number(&mut self) -> Web3Result<u64> {
        let req = Web3Call::BlockNumber(());

        let res: RpcResponse<U64> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(Web3Error::EmptyResponse)?;

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
}
