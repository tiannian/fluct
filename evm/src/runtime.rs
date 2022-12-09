use evm::{
    executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata},
    Config, ExitReason,
};
use fluct_core::{KeyValueStoreReadonly, Store};
use primitive_types::{H160, U256, H256};

use crate::{CoreBackend, CoreVicinity, Precompiles};

pub fn call<KV: KeyValueStoreReadonly>(
    from: H160,
    to: H160,
    value: U256,
    data: Vec<u8>,
    gas_limit: u64,
    store: &Store<KV>,
    vicinity: CoreVicinity,
) -> (ExitReason, Vec<u8>) {
    let config = Config::istanbul();
    let metadata = StackSubstateMetadata::new(gas_limit, &config);
    let backend = CoreBackend::new(&store.block, &store.state, vicinity, from);
    let state = MemoryStackState::new(metadata, &backend);
    let precompiles = Precompiles::default();
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

    executor.transact_call(from, to, value, data, gas_limit, Vec::new())
}

pub fn create<KV: KeyValueStoreReadonly>(
    from: H160,
    value: U256,
    init_code: Vec<u8>,
    gas_limit: u64,
    store: &Store<KV>,
    vicinity: CoreVicinity,
) -> (ExitReason, Vec<u8>) {
    let config = Config::istanbul();
    let metadata = StackSubstateMetadata::new(gas_limit, &config);
    let backend = CoreBackend::new(&store.block, &store.state, vicinity, from);
    let state = MemoryStackState::new(metadata, &backend);
    let precompiles = Precompiles::default();
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

    executor.transact_create(from, value, init_code, gas_limit, Vec::new())
}

pub fn create2<KV: KeyValueStoreReadonly>(
    from: H160,
    value: U256,
    init_code: Vec<u8>,
    salt: H256,
    gas_limit: u64,
    store: &Store<KV>,
    vicinity: CoreVicinity,
) -> (ExitReason, Vec<u8>) {
    let config = Config::istanbul();
    let metadata = StackSubstateMetadata::new(gas_limit, &config);
    let backend = CoreBackend::new(&store.block, &store.state, vicinity, from);
    let state = MemoryStackState::new(metadata, &backend);
    let precompiles = Precompiles::default();
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

    executor.transact_create2(from, value, init_code, salt, gas_limit, Vec::new())
}

