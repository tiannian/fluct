use alloc::vec::Vec;
use ethereum::EIP658ReceiptData;
use primitive_types::H256;

use crate::{Error, KeyValueStore, KeyValueStoreReadonly, Result};

/// Receipt store
pub struct ReceiptStore<KV> {
    /// receipthash -> receipt
    receipt: KV,

    /// topic-txhash -> 1
    log_index: KV,
}

impl<KV: KeyValueStoreReadonly> ReceiptStore<KV> {
    const PLACEHOLDER_EXIST: [u8; 1] = [1u8];

    pub fn get_receipt(&self, txhash: H256) -> Result<Option<EIP658ReceiptData>> {
        if let Some(data) = self.receipt.get(txhash).map_err(Error::store)? {
            let receipt = rlp::decode(&data)?;
            Ok(Some(receipt))
        } else {
            Ok(None)
        }
    }

    pub fn iterate_topic(&self, topic: H256) -> Result<Vec<H256>> {
        let mut end = topic.to_fixed_bytes().to_vec();
        let max = H256::repeat_byte(0xff);
        end.extend_from_slice(max.as_bytes());
        let iter = self.log_index.range(topic, end, false);

        let mut res = Vec::new();

        for it in iter {
            let (k, _) = it.map_err(Error::store)?;
            let txhash = H256::from_slice(&k[32..]);
            res.push(txhash);
        }

        Ok(res)
    }
}

impl<KV: KeyValueStore> ReceiptStore<KV> {
    pub fn add_receipt(&self, txhash: H256, receipt: EIP658ReceiptData) -> Result<()> {
        let ops = [(txhash, Some(rlp::encode(&receipt).to_vec()))];
        self.receipt.ops(&ops).map_err(Error::store)?;

        for event in receipt.logs {
            for topic in event.topics {
                let mut key = topic.as_bytes().to_vec();
                key.extend_from_slice(txhash.as_bytes());

                self.log_index
                    .set(key, Self::PLACEHOLDER_EXIST.to_vec())
                    .map_err(Error::store)?;
            }
        }

        Ok(())
    }
}
