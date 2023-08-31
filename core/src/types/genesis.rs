use ethers_core::types::{Bytes, H256, U256};
use serde::{Deserialize, Serialize};

use crate::{Parser, Transaction};

#[derive(Debug, Serialize, Deserialize)]
pub struct Genesis<T, E> {
    pub consensus: ConsensusGenesis<T>,
    pub execution: E,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsensusGenesis<T> {
    pub chain_id: u64,
    pub earliest_block_height: u64,
    pub earliest_block_hash: H256,
    pub extra_data: Bytes,
    pub block_gas_limit: U256,
    pub block_block_size: u64,
    pub transactions: Vec<T>,
    pub timestamp: u64,
}

impl<E> Genesis<Bytes, E> {
    pub fn from_transaction<P>(self) -> Result<Genesis<Transaction, E>, P::Error>
    where
        P: Parser,
    {
        let mut transactions = Vec::with_capacity(self.consensus.transactions.len());

        for tx in &self.consensus.transactions {
            transactions.push(P::deserialize_transaction(tx)?);
        }

        Ok(Genesis {
            consensus: (self.consensus, transactions).into(),
            execution: self.execution,
        })
    }
}

impl From<(ConsensusGenesis<Bytes>, Vec<Transaction>)> for ConsensusGenesis<Transaction> {
    fn from(value: (ConsensusGenesis<Bytes>, Vec<Transaction>)) -> Self {
        Self {
            chain_id: value.0.chain_id,
            earliest_block_height: value.0.earliest_block_height,
            earliest_block_hash: value.0.earliest_block_hash,
            extra_data: value.0.extra_data,
            block_gas_limit: value.0.block_gas_limit,
            block_block_size: value.0.block_block_size,
            transactions: value.1,
            timestamp: value.0.timestamp,
        }
    }
}
