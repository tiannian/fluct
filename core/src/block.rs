use ethereum::Header;
use ethers_core::types::{Block as EthBlock, H256};

use crate::Transaction;

#[derive(Debug, Clone)]
pub enum Block {
    Ethereum(EthBlock<Transaction>),
}

impl Block {
    pub fn fill_hash(&mut self, hash: H256) {
        match self {
            Self::Ethereum(v) => v.hash = Some(hash),
        }
    }

    pub fn hash(&self) -> Option<H256> {
        match self {
            Self::Ethereum(v) => v.hash,
        }
    }
}

pub enum BlockHeader {
    Ethereum(Header),
}

impl From<Block> for BlockHeader {
    fn from(value: Block) -> Self {
        match value {
            Block::Ethereum(v) => v.into(),
        }
    }
}

impl<TX> From<EthBlock<TX>> for BlockHeader {
    fn from(b: EthBlock<TX>) -> Self {
        let header = Header {
            parent_hash: b.parent_hash,
            ommers_hash: b.uncles_hash,
            beneficiary: b.author.unwrap_or_default(),
            state_root: b.state_root,
            transactions_root: b.transactions_root,
            receipts_root: b.receipts_root,
            logs_bloom: b.logs_bloom.unwrap_or_default(),
            difficulty: b.difficulty,
            number: b.number.unwrap_or_default().as_u64().into(),
            gas_limit: b.gas_limit,
            gas_used: b.gas_used,
            timestamp: b.timestamp.as_u64(),
            extra_data: b.extra_data.0.into(),
            mix_hash: b.mix_hash.unwrap_or_default(),
            nonce: b.nonce.unwrap_or_default(),
        };

        Self::Ethereum(header)
    }
}

impl BlockHeader {
    pub fn hash(&self) -> H256 {
        match self {
            Self::Ethereum(v) => v.hash(),
        }
    }
}
