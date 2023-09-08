use async_trait::async_trait;
use ethers_core::types::{
    Block, BlockId, Bytes, SyncingStatus, TransactionReceipt, H160, H256, U256,
};
use serde::{Deserialize, Serialize};

use crate::{types, EngineError, Service, Transaction, Web3Error};

/// Service of Execution Engine
#[async_trait]
pub trait ExecutionService: Service {
    /// Api instance of Engine API
    type EngineApi: EngineApi;

    type Web3Api: Web3Api;

    /// Execution genesis type
    type Genesis: Serialize + for<'de> Deserialize<'de>;

    /// Create engine api instance
    fn engine_api(&self) -> Result<Self::EngineApi, Self::Error>;

    /// Create web3 api instance
    fn web3_api(&self) -> Result<Self::Web3Api, Self::Error>;

    /// Init chain
    fn init(&mut self, genesis: Self::Genesis) -> Result<(), Self::Error>;

    /// Remove all data in engine.
    ///
    /// This method only can call in devnode.
    fn reset(&mut self) -> Result<(), Self::Error>;
}

/// Api of Engine
#[async_trait]
pub trait EngineApi {
    /// Choice block chain fork.
    ///
    /// Spec: [`engine_forkchoiceUpdatedV1`](https://github.com/ethereum/execution-apis/blob/769c53c94c4e487337ad0edea9ee0dce49c79bfa/src/engine/specification.md#engine_forkchoiceupdatedv1)
    async fn engine_fork_choice(
        &mut self,
        state: types::ForkChoiceState,
        attr: types::PayloadAttributes<Transaction>,
    ) -> Result<types::ForkChoiceResult, EngineError>;

    /// Add block on blockchain
    ///
    /// Spec: [`engine_newPayloadV1`](https://github.com/ethereum/execution-apis/blob/769c53c94c4e487337ad0edea9ee0dce49c79bfa/src/engine/specification.md#engine_newpayloadv1)
    async fn engine_new_payload(
        &mut self,
        payload: types::ExecutionPayload<Transaction>,
    ) -> Result<types::Status, EngineError>;

    /// Get added block
    async fn engine_get_payload(
        &mut self,
        payload_id: Bytes,
    ) -> Result<types::ExecutionPayload<Transaction>, EngineError>;
}

/// Api of web3
#[async_trait]
pub trait Web3Api {
    /// Get latest block number
    async fn block_number(&mut self) -> Result<u64, Web3Error>;

    /// Get chain_id
    async fn chain_id(&mut self) -> Result<u64, Web3Error>;

    /// Get block by hash, number or tag
    async fn get_block(&mut self, block: BlockId) -> Result<Option<Block<Transaction>>, Web3Error>;

    /// Get trnsaction by hash
    async fn get_transaction(&mut self, hash: H256) -> Result<Option<Transaction>, Web3Error>;

    /// Get Transaction receipt by transaction hash
    async fn get_transaction_receipt(
        &mut self,
        hash: H256,
    ) -> Result<Option<TransactionReceipt>, Web3Error>;

    /// Get syncing statue
    async fn syncing(&mut self) -> Result<SyncingStatus, Web3Error>;

    /// Get account balance
    async fn balance(&mut self, address: H160, block: Option<BlockId>) -> Result<U256, Web3Error>;

    /// Get account code
    async fn code(&mut self, address: H160, block: Option<BlockId>) -> Result<Bytes, Web3Error>;

    /// Get account storage
    async fn storage_at(
        &mut self,
        address: H160,
        index: H256,
        block: Option<BlockId>,
    ) -> Result<H256, Web3Error>;
}
