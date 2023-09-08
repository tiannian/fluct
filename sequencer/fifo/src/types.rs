use ethers_core::types::H256;
use fluct_core::Transaction;

pub enum ApiRequest {
    Transaction(Transaction),
    TxHash(H256),
    GetAllTransaction,
}

pub enum ApiResponse {
    GetAllTransaction(Vec<Transaction>),
}
