use ethereum::Header;
use ethereum_types::H256;

use crate::{Error, KeyValueDb, KeyValueStore, KeyValueStoreReadonly, Result};

#[derive(Clone)]
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
    pub const LATEST_HEIGHT: [u8; 8] = [0xff; 8];
}

impl<KV: KeyValueStoreReadonly> BlockStore<KV> {
    pub fn latest_hash(&self) -> Result<u64> {
        let height = if let Some(data) = self
            .blockheight
            .get(&Self::LATEST_HEIGHT)
            .map_err(Error::store)?
        {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&data);
            u64::from_be_bytes(bytes)
        } else {
            0
        };

        Ok(height)
    }

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

    pub fn block_hash_by_height(&self, height: u64) -> Result<Option<H256>> {
        if let Some(data) = self
            .blockheight
            .get(&height.to_le_bytes())
            .map_err(Error::store)?
        {
            Ok(Some(H256::from_slice(&data)))
        } else {
            Ok(None)
        }
    }

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
    pub fn add_block(&self, header: &Header, txhashes: &[H256]) -> Result<()> {
        let hash = header.hash();

        let bytes = rlp::encode(header);
        let ops = &[(hash.as_bytes(), Some(bytes.into()))];
        self.blockheader.ops(ops).map_err(Error::store)?;

        let number = header.number.as_u64().to_le_bytes();
        let ops = &[
            (&number, Some(hash.to_fixed_bytes().into())),
            (&Self::LATEST_HEIGHT, Some(number.into())),
        ];
        self.blockheight.ops(ops).map_err(Error::store)?;

        let bytes = rlp::encode_list(txhashes);
        let ops = &[(hash.as_bytes(), Some(bytes.into()))];
        self.txs.ops(ops).map_err(Error::store)?;

        Ok(())
    }
}
