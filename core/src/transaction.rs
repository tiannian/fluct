use ethers_core::types::{
    Eip1559TransactionRequest, Eip2930TransactionRequest, TransactionRequest, H256, U256,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositedTransactionRequest {
    pub tx: TransactionRequest,
    pub source_hash: H256,
    pub mint: U256,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Transaction {
    Legacy(TransactionRequest),
    Eip2930(Eip2930TransactionRequest),
    Eip1559(Eip1559TransactionRequest),
    Deposited(DepositedTransactionRequest),
}
