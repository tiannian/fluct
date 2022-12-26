use alloc::vec::Vec;
use ethereum::Header;
use primitive_types::H256;

use crate::{utils, Error, KeyValueDb, KeyValueStore, KeyValueStoreReadonly, Result};

/// Block store
pub struct BlockStore<KV> {
    /// Store height -> hash
    ///
    /// height encode as bytes
    /// height = 0xFFFFFFFFFFFFFFFF means latest
    pub(crate) blockheight: KV,

    /// Store hash -> header
    pub(crate) blockheader: KV,

    /// Block Hash -> txs
    pub(crate) txs: KV,
}

/// Open readonly block store
pub fn open_block_store_readonly<Db: KeyValueDb>(
    db: &Db,
) -> Result<BlockStore<Db::KeyValueStoreReadonly>> {
    let blockheader = db.open_readonly("block_header").map_err(Error::store)?;
    let blockheight = db.open_readonly("block_height").map_err(Error::store)?;
    let txs = db.open_readonly("block_txs").map_err(Error::store)?;

    Ok(BlockStore {
        blockheight,
        blockheader,
        txs,
    })
}

/// Open writeable block store
pub fn open_block_store<Db: KeyValueDb>(db: &Db) -> Result<BlockStore<Db::KeyValueStore>> {
    let blockheader = db.open("block_header").map_err(Error::store)?;
    let blockheight = db.open("block_height").map_err(Error::store)?;
    let txs = db.open("block_txs").map_err(Error::store)?;

    Ok(BlockStore {
        blockheight,
        blockheader,
        txs,
    })
}

impl<KV> BlockStore<KV> {
    const LATEST_HEIGHT: [u8; 8] = [0xff; 8];
}

impl<KV: KeyValueStoreReadonly> BlockStore<KV> {
    /// Get laste hash
    pub fn latest_hash(&self) -> Result<u64> {
        let height = if let Some(data) = self
            .blockheight
            .get(Self::LATEST_HEIGHT)
            .map_err(Error::store)?
        {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&data);
            utils::u64_from_bytes(bytes)
        } else {
            0
        };

        Ok(height)
    }

    /// Get block by hash
    pub fn block_by_hash(&self, hash: H256) -> Result<Option<Header>> {
        if let Some(data) = self
            .blockheader
            .get(hash.as_bytes())
            .map_err(Error::store)?
        {
            Ok(Some(rlp::decode(&data)?))
        } else {
            Ok(None)
        }
    }

    /// Get block hash of height
    pub fn block_hash_by_height(&self, height: u64) -> Result<Option<H256>> {
        if let Some(data) = self
            .blockheight
            .get(utils::u64_to_bytes(&height))
            .map_err(Error::store)?
        {
            Ok(Some(H256::from_slice(&data)))
        } else {
            Ok(None)
        }
    }

    /// Get block's transactions
    pub fn get_txs(&self, hash: H256) -> Result<Vec<H256>> {
        if let Some(data) = self.txs.get(hash.as_bytes()).map_err(Error::store)? {
            let txs = rlp::decode_list(&data);

            Ok(txs)
        } else {
            Ok(Vec::new())
        }
    }
}

impl<KV: KeyValueStore> BlockStore<KV> {
    /// Add block's
    pub fn add_block(&self, header: &Header, txhashes: &[H256]) -> Result<()> {
        let hash = header.hash();

        let bytes = rlp::encode(header);
        self.blockheader
            .set(hash.as_bytes(), bytes.into())
            .map_err(Error::store)?;

        let number = utils::u64_to_bytes(&header.number.as_u64());
        self.blockheight
            .set(number, hash.to_fixed_bytes().into())
            .map_err(Error::store)?;
        self.blockheight
            .set(Self::LATEST_HEIGHT, number.into())
            .map_err(Error::store)?;

        let bytes = rlp::encode_list(txhashes);
        self.txs
            .set(hash.as_bytes(), bytes.into())
            .map_err(Error::store)?;

        Ok(())
    }
}
