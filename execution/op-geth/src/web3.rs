use async_trait::async_trait;
use ethers_core::types::{
    Block, BlockId, BlockNumber, Bytes, SyncingStatus, TransactionReceipt, H160, H256, U256, U64,
};
use fluct_core::{Transaction, Web3Api, Web3Error, Web3Result};
use fluct_jsonrpc::client::{RpcClient, RpcResponse};
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Clone)]
pub struct GethWeb3Api {
    client: RpcClient,
}

impl GethWeb3Api {
    pub fn new() -> Result<Self, Error> {
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
    #[serde(rename = "eth_getTransactionByHash")]
    GetTransactionByHash((H256,)),
    #[serde(rename = "eth_getTransactionReceipt")]
    GetTransactionReceipt((H256,)),
    #[serde(rename = "eth_getBalance")]
    Balance((H160,)),
    #[serde(rename = "eth_getBalance")]
    BalanceWithBlock((H160, BlockId)),
    #[serde(rename = "eth_getCode")]
    Code((H160,)),
    #[serde(rename = "eth_getCode")]
    CodeWithBlock((H160, BlockId)),
    #[serde(rename = "eth_getStorageAt")]
    Storage((H160, H256)),
    #[serde(rename = "eth_getStorageAt")]
    StorageWithBlock((H160, H256, BlockId)),
    #[serde(rename = "eth_syncing")]
    Syncing(()),
}

#[async_trait]
impl Web3Api for GethWeb3Api {
    async fn block_number(&mut self) -> Web3Result<u64> {
        let req = Web3Call::BlockNumber(());

        let res: RpcResponse<U64> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(Web3Error::EmptyResponse)?;

        Ok(res.as_u64())
    }

    async fn chain_id(&mut self) -> Result<u64, Web3Error> {
        let req = Web3Call::ChainId(());

        let res: RpcResponse<U64> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(Web3Error::EmptyResponse)?;

        Ok(res.as_u64())
    }

    async fn get_block(&mut self, block: BlockId) -> Result<Option<Block<Transaction>>, Web3Error> {
        let req = match block {
            BlockId::Hash(v) => Web3Call::GetBlockByHash((v, true)),
            BlockId::Number(v) => Web3Call::GetBlockByNumber((v, true)),
        };

        let res: RpcResponse<Block<Transaction>> = self.client.call(req).await?;
        let res = res.into_result()?;

        Ok(res)
    }

    async fn get_transaction(&mut self, hash: H256) -> Result<Option<Transaction>, Web3Error> {
        let req = Web3Call::GetTransactionByHash((hash,));

        let res: RpcResponse<Transaction> = self.client.call(req).await?;
        let res = res.into_result()?;

        Ok(res)
    }

    async fn get_transaction_receipt(
        &mut self,
        hash: H256,
    ) -> Result<Option<TransactionReceipt>, Web3Error> {
        let req = Web3Call::GetTransactionReceipt((hash,));

        let res: RpcResponse<TransactionReceipt> = self.client.call(req).await?;
        let res = res.into_result()?;

        Ok(res)
    }

    async fn syncing(&mut self) -> Result<SyncingStatus, Web3Error> {
        let req = Web3Call::Syncing(());

        let res: RpcResponse<SyncingStatus> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(Web3Error::EmptyResponse)?;

        Ok(res)
    }

    async fn balance(&mut self, address: H160, block: Option<BlockId>) -> Result<U256, Web3Error> {
        let req = if let Some(b) = block {
            Web3Call::BalanceWithBlock((address, b))
        } else {
            Web3Call::Balance((address,))
        };

        let res: RpcResponse<U256> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(Web3Error::EmptyResponse)?;

        Ok(res)
    }

    /// Get account code
    async fn code(&mut self, address: H160, block: Option<BlockId>) -> Result<Bytes, Web3Error> {
        let req = if let Some(b) = block {
            Web3Call::CodeWithBlock((address, b))
        } else {
            Web3Call::Code((address,))
        };

        let res: RpcResponse<Bytes> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(Web3Error::EmptyResponse)?;

        Ok(res)
    }

    /// Get account storage
    async fn storage_at(
        &mut self,
        address: H160,
        index: H256,
        block: Option<BlockId>,
    ) -> Result<H256, Web3Error> {
        let req = if let Some(b) = block {
            Web3Call::StorageWithBlock((address, index, b))
        } else {
            Web3Call::Storage((address, index))
        };

        let res: RpcResponse<H256> = self.client.call(req).await?;
        let res = res.into_result()?.ok_or(Web3Error::EmptyResponse)?;

        Ok(res)
    }
}
