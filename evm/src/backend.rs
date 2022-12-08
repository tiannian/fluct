use evm::backend::{Backend, Basic};
use fluct_core::{BlockStore, KeyValueStoreReadonly, StateStore};
use primitive_types::{H160, H256, U256};

use crate::CoreVicinity;

pub struct CoreBackend<'a, KV> {
    block: &'a BlockStore<KV>,
    state: &'a StateStore<KV>,
    vicinity: CoreVicinity,
}

impl<'a, KV> CoreBackend<'a, KV> {
    pub fn new(
        block: &'a BlockStore<KV>,
        state: &'a StateStore<KV>,
        vicinity: CoreVicinity,
    ) -> Self {
        CoreBackend {
            block,
            state,
            vicinity,
        }
    }
}

impl<'a, KV> Backend for CoreBackend<'a, KV>
where
    KV: KeyValueStoreReadonly,
{
    fn gas_price(&self) -> U256 {
        self.vicinity.gas_price
    }

    fn origin(&self) -> H160 {
        self.vicinity.origin
    }

    fn block_hash(&self, number: U256) -> H256 {
        if number > U256::from(self.vicinity.block_height) {
            H256::default()
        } else {
            self.block
                .block_hash_by_height(number.as_u64())
                .expect("Failed to get block height")
                .unwrap_or_default()
        }
    }

    fn block_number(&self) -> U256 {
        self.vicinity.block_height.into()
    }

    fn block_coinbase(&self) -> H160 {
        self.vicinity.block_coinbase
    }

    fn block_timestamp(&self) -> U256 {
        self.vicinity.block_timestamp
    }

    fn block_difficulty(&self) -> U256 {
        self.vicinity.block_difficulty
    }

    fn block_gas_limit(&self) -> U256 {
        self.vicinity.block_gas_limit
    }

    fn block_base_fee_per_gas(&self) -> U256 {
        self.vicinity.block_base_fee_per_gas
    }

    fn chain_id(&self) -> U256 {
        self.vicinity.chain_id
    }

    fn basic(&self, address: H160) -> Basic {
        let (balance, nonce) = self
            .state
            .get_basic(address, self.vicinity.block_height)
            .expect("Failed to read basic")
            .unwrap_or_default();

        Basic { balance, nonce }
    }

    fn code(&self, address: H160) -> Vec<u8> {
        self.state
            .get_code(address, self.vicinity.block_height)
            .expect("Failed to get code")
            .unwrap_or_default()
    }

    fn exists(&self, address: H160) -> bool {
        self.state
            .get_basic(address, self.vicinity.block_height)
            .expect("Failed to get basic")
            .is_some()
    }

    fn storage(&self, address: H160, index: H256) -> H256 {
        self.original_storage(address, index).unwrap_or_default()
    }

    fn original_storage(&self, address: H160, index: H256) -> Option<H256> {
        self.state
            .get_storage(address, index, self.vicinity.block_height)
            .expect("Failed to get storage")
    }
}
