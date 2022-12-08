use ethereum_types::{H160, H256, U256};
use evm::backend::{Backend, Basic};
use fluct_core::{BlockStore, KeyValueStoreReadonly, StateStore};

pub struct CoreVicinity {
    pub gas_price: U256,
    pub origin: H160,
    pub height: u64,
    pub coinbase: H160,
    pub timestamp: U256,
    pub difficulty: U256,
    pub gas_limit: U256,
    pub base_fee_per_gas: U256,
    pub chain_id: U256,
}

pub struct CoreBackend<KV> {
    block: BlockStore<KV>,
    state: StateStore<KV>,
    vicinity: CoreVicinity,
}

impl<KV> CoreBackend<KV> {
    pub fn new(block: BlockStore<KV>, state: StateStore<KV>, vicinity: CoreVicinity) -> Self {
        CoreBackend {
            block,
            state,
            vicinity,
        }
    }
}

impl<KV: KeyValueStoreReadonly> Backend for CoreBackend<KV> {
    fn gas_price(&self) -> U256 {
        self.vicinity.gas_price
    }

    fn origin(&self) -> H160 {
        self.vicinity.origin
    }

    fn block_hash(&self, number: U256) -> H256 {
        if number > U256::from(self.vicinity.height) {
            H256::default()
        } else {
            self.block
                .block_hash_by_height(number.as_u64())
                .expect("Failed to get block height")
                .unwrap_or_default()
        }
    }

    fn block_number(&self) -> U256 {
        self.vicinity.height.into()
    }

    fn block_coinbase(&self) -> H160 {
        self.vicinity.coinbase
    }

    fn block_timestamp(&self) -> U256 {
        self.vicinity.timestamp
    }

    fn block_difficulty(&self) -> U256 {
        self.vicinity.difficulty
    }

    fn block_gas_limit(&self) -> U256 {
        self.vicinity.gas_limit
    }

    fn block_base_fee_per_gas(&self) -> U256 {
        self.vicinity.base_fee_per_gas
    }

    fn chain_id(&self) -> U256 {
        self.vicinity.chain_id
    }

    fn basic(&self, address: H160) -> Basic {
        let (balance, nonce) = self
            .state
            .get_basic(address, self.vicinity.height)
            .expect("Failed to read basic")
            .unwrap_or_default();

        Basic { balance, nonce }
    }

    fn code(&self, address: H160) -> Vec<u8> {
        self.state
            .get_code(address, self.vicinity.height)
            .expect("Failed to get code")
            .unwrap_or_default()
    }

    fn exists(&self, address: H160) -> bool {
        self.state
            .get_basic(address, self.vicinity.height)
            .expect("Failed to get basic")
            .is_some()
    }

    fn storage(&self, address: H160, index: H256) -> H256 {
        self.original_storage(address, index).unwrap_or_default()
    }

    fn original_storage(&self, address: H160, index: H256) -> Option<H256> {
        self.state
            .get_storage(address, index, self.vicinity.height)
            .expect("Failed to get storage")
    }
}
