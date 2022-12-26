use ethereum::{EnvelopedDecodable, EnvelopedEncodable, TransactionAny};
use primitive_types::H256;

use crate::{Error, KeyValueDb, KeyValueStore, KeyValueStoreReadonly, Result};

/// Transaction store
pub struct TxStore<KV> {
    /// txhash -> tx
    pub(crate) tx: KV,

    /// txhash -> blockhash
    pub(crate) tx_meta: KV,
}

/// Open a readonly transaction store
pub fn open_tx_store_readonly<Db: KeyValueDb>(
    db: &Db,
) -> Result<TxStore<Db::KeyValueStoreReadonly>> {
    let tx = db.open_readonly("tx").map_err(Error::store)?;
    let tx_meta = db.open_readonly("tx_meta").map_err(Error::store)?;

    Ok(TxStore { tx, tx_meta })
}

/// Open a writeable transaction store
pub fn open_tx_store<Db: KeyValueDb>(db: &Db) -> Result<TxStore<Db::KeyValueStore>> {
    let tx = db.open("tx").map_err(Error::store)?;
    let tx_meta = db.open("tx_meta").map_err(Error::store)?;

    Ok(TxStore { tx, tx_meta })
}

impl<KV: KeyValueStoreReadonly> TxStore<KV> {
    /// Get transaction body
    pub fn get_body(&self, txhash: H256) -> Result<Option<TransactionAny>> {
        if let Some(data) = self.tx.get(txhash.as_bytes()).map_err(Error::store)? {
            Ok(Some(
                TransactionAny::decode(&data).map_err(Error::EthereumEnvelopedError)?,
            ))
        } else {
            Ok(None)
        }
    }

    /// Get transaction's block hash
    pub fn get_block_hash(&self, txhash: H256) -> Result<Option<H256>> {
        if let Some(data) = self.tx_meta.get(txhash.as_bytes()).map_err(Error::store)? {
            Ok(Some(H256::from_slice(&data)))
        } else {
            Ok(None)
        }
    }
}

impl<KV: KeyValueStore> TxStore<KV> {
    /// Add transaction to store
    pub fn add_tx(&self, tx: &TransactionAny, txhash: H256, block_hash: H256) -> Result<()> {
        let bytes = tx.encode();

        self.tx
            .set(txhash.as_bytes(), bytes.into())
            .map_err(Error::store)?;
        self.tx_meta
            .set(txhash.as_bytes(), block_hash.0.into())
            .map_err(Error::store)?;

        Ok(())
    }
}
