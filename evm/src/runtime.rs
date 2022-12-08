use evm::{
    executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata},
    Config, ExitReason,
};
use fluct_core::{KeyValueStoreReadonly, StateStore, Store};
use primitive_types::{H160, U256};

use crate::CoreBackend;

pub fn call<KV: KeyValueStoreReadonly>(
    from: H160,
    to: H160,
    value: U256,
    data: Vec<u8>,
    gas_limit: u64,
    store: &Store<KV>,
) {
    let config = Config::istanbul();
    let metadata = StackSubstateMetadata::new(gas_limit, &config);

    // let backend = CoreBackend::new(&store.block, &store.state, vicinity);

    // let state = MemoryStackState::new(metadata, backend);

    // let executor = StackExecutor::new_with_precompiles(state, config, precompile_set)
}
