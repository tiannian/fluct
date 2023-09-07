use ethers_core::types::H256;
use fluct_core::Transaction;

pub enum ApiMessage {
    Transaction(Transaction),
    TxHash(H256),
}
