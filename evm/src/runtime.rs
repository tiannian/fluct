use evm::{
    executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata, Log},
    Config, ExitReason, backend::Apply,
};
use fluct_core::{KeyValueStoreReadonly, Store};
use primitive_types::{H160, H256, U256};

use crate::{CoreBackend, CoreVicinity, Precompiles, AddressRecorder};

pub struct Runtime<'a, KV, R> {
    pub(crate) config: Config,
    pub(crate) vicinity: CoreVicinity,
    pub(crate) recoder: Option<R>,
    pub(crate) store: &'a Store<KV>,
    pub applies: Vec<Apply<Vec<(H256, H256)>>>,
    pub logs: Vec<Log>,
}

impl<'a, KV, R> Runtime<'a, KV, R>
where
    KV: KeyValueStoreReadonly,
    R: AddressRecorder,
{
    pub fn call(
        &mut self,
        from: H160,
        to: H160,
        value: U256,
        data: Vec<u8>,
        gas_limit: u64,
    ) -> (ExitReason, Vec<u8>) {
        let metadata = StackSubstateMetadata::new(gas_limit, &self.config);
        let backend = CoreBackend::new(&self.store.block, &self.store.state, &self.vicinity, from, self.recoder.as_mut());
        let state = MemoryStackState::new(metadata, &backend);
        let precompiles = Precompiles::default();
        let mut executor = StackExecutor::new_with_precompiles(state, &self.config, &precompiles);
        let res = executor.transact_call(from, to, value, data, gas_limit, Vec::new());

        let (applies, logs) = executor.into_state().deconstruct();

        res
    }

/*     pub fn create<KV: KeyValueStoreReadonly>( */
    /*     from: H160, */
    /*     value: U256, */
    /*     init_code: Vec<u8>, */
    /*     gas_limit: u64, */
    /*     store: &Store<KV>, */
    /*     vicinity: CoreVicinity, */
    /* ) -> (ExitReason, Vec<u8>) { */
    /*     let config = Config::istanbul(); */
    /*     let metadata = StackSubstateMetadata::new(gas_limit, &config); */
    /*     let backend = CoreBackend::new(&store.block, &store.state, vicinity, from); */
    /*     let state = MemoryStackState::new(metadata, &backend); */
    /*     let precompiles = Precompiles::default(); */
    /*     let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles); */
    /*  */
    /*     executor.transact_create(from, value, init_code, gas_limit, Vec::new()) */
    /* } */
    /*  */
    /* pub fn create2<KV: KeyValueStoreReadonly>( */
    /*     from: H160, */
    /*     value: U256, */
    /*     init_code: Vec<u8>, */
    /*     salt: H256, */
    /*     gas_limit: u64, */
    /*     store: &Store<KV>, */
    /*     vicinity: CoreVicinity, */
    /* ) -> (ExitReason, Vec<u8>) { */
    /*     let config = Config::istanbul(); */
    /*     let metadata = StackSubstateMetadata::new(gas_limit, &config); */
    /*     let backend = CoreBackend::new(&store.block, &store.state, vicinity, from); */
    /*     let state = MemoryStackState::new(metadata, &backend); */
    /*     let precompiles = Precompiles::default(); */
    /*     let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles); */
    /*  */
    /*     executor.transact_create2(from, value, init_code, salt, gas_limit, Vec::new()) */
    /* } */
}
