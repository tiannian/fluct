pub use ethers_core::types::Transaction;

/// Utils for transaction
pub mod transaction_utils {
    use ethers_core::{
        types::Bytes,
        utils::rlp::{Decodable, Rlp},
    };

    use crate::{Error, Transaction};

    /// Encode transaction to bytes
    pub fn transaction_to_bytes(txs: &[Transaction]) -> Vec<Bytes> {
        let mut ret = Vec::with_capacity(txs.len());

        for tx in txs {
            ret.push(tx.rlp())
        }

        ret
    }

    /// Decode bytes to transaction
    pub fn bytes_to_transaction(bytes: &[Bytes]) -> Result<Vec<Transaction>, Error> {
        let mut ret = Vec::with_capacity(bytes.len());

        for bytes in bytes {
            let rlp = Rlp::new(bytes);
            let tx = Transaction::decode(&rlp)?;
            ret.push(tx)
        }

        Ok(ret)
    }
}
