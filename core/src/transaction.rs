use ethers_core::types::{
    Eip1559TransactionRequest, Eip2930TransactionRequest, Signature, TransactionRequest, H256, U256,
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
    Legacy(TransactionRequest, Signature),
    Eip2930(Eip2930TransactionRequest, Signature),
    Eip1559(Eip1559TransactionRequest, Signature),
    Deposited(DepositedTransactionRequest),
}

pub mod transaction_utils {
    use ethers_core::types::Bytes;

    use crate::{Parser, Transaction};

    pub fn transaction_to_bytes<P>(txs: &[Transaction]) -> Result<Vec<Bytes>, P::Error>
    where
        P: Parser,
    {
        let mut ret = Vec::with_capacity(txs.len());

        for tx in txs {
            ret.push(P::serialize_transaction(tx).into());
        }

        Ok(ret)
    }

    pub fn bytes_to_transaction<P>(txs: &[Bytes]) -> Result<Vec<Transaction>, P::Error>
    where
        P: Parser,
    {
        let mut ret = Vec::with_capacity(txs.len());

        for tx in txs {
            ret.push(P::deserialize_transaction(tx)?);
        }

        Ok(ret)
    }
}
