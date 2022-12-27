use core::mem;

use evm::{
    executor::stack::{Log, StackExecutor, StackSubstateMetadata},
    Config, ExitReason,
};
use fluct_core::{KeyValueStore, KeyValueStoreReadonly, Store};
use primitive_types::{H160, H256, U256};

use crate::{
    stack::{CoreBackend, CoreStackState, State},
    AddressRecorder, CoreVicinity, Precompiles, Result,
};

pub struct Runtime<'a, KV, R> {
    pub(crate) config: Config,
    pub(crate) vicinity: CoreVicinity,
    pub recoder: Option<R>,
    pub(crate) store: &'a Store<KV>,
    pub(crate) state: Option<State>,
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
    ) -> (ExitReason, Vec<u8>, State) {
        let metadata = StackSubstateMetadata::new(gas_limit, &self.config);
        let backend = CoreBackend::new(
            &self.store.block,
            &self.store.state,
            &self.vicinity,
            from,
            self.recoder.as_mut(),
        );
        let s = mem::take(&mut self.state);
        let state = CoreStackState::new(&backend, metadata, s);
        let precompiles = Precompiles::default();
        let mut executor = StackExecutor::new_with_precompiles(state, &self.config, &precompiles);
        let (e, d) = executor.transact_call(from, to, value, data, gas_limit, Vec::new());

        let state = executor.into_state().deconstruct();

        (e, d, state)
    }

    pub fn create(
        &mut self,
        from: H160,
        value: U256,
        init_code: Vec<u8>,
        gas_limit: u64,
    ) -> (ExitReason, Vec<u8>, State) {
        let metadata = StackSubstateMetadata::new(gas_limit, &self.config);
        let backend = CoreBackend::new(
            &self.store.block,
            &self.store.state,
            &self.vicinity,
            from,
            self.recoder.as_mut(),
        );
        let s = mem::take(&mut self.state);
        let state = CoreStackState::new(&backend, metadata, s);
        let precompiles = Precompiles::default();
        let mut executor = StackExecutor::new_with_precompiles(state, &self.config, &precompiles);

        let (er, d) = executor.transact_create(from, value, init_code, gas_limit, Vec::new());

        let state = executor.into_state().deconstruct();

        (er, d, state)
    }

    pub fn create2(
        &mut self,
        from: H160,
        value: U256,
        init_code: Vec<u8>,
        salt: H256,
        gas_limit: u64,
    ) -> (ExitReason, Vec<u8>, State) {
        let metadata = StackSubstateMetadata::new(gas_limit, &self.config);
        let backend = CoreBackend::new(
            &self.store.block,
            &self.store.state,
            &self.vicinity,
            from,
            self.recoder.as_mut(),
        );
        let s = mem::take(&mut self.state);
        let state = CoreStackState::new(&backend, metadata, s);
        let precompiles = Precompiles::default();
        let mut executor = StackExecutor::new_with_precompiles(state, &self.config, &precompiles);

        let (e, d) = executor.transact_create2(from, value, init_code, salt, gas_limit, Vec::new());

        let state = executor.into_state().deconstruct();

        (e, d, state)
    }
}

fn remove<KV>(store: &Store<KV>, addr: H160) -> Result<()>
where
    KV: KeyValueStore,
{
    store.state.del_basic(addr)?;
    store.state.del_code(addr)?;
    Ok(())
}

pub fn apply<KV>(store: Store<KV>, state: State) -> Result<Vec<Log>>
where
    KV: KeyValueStore,
{
    let mut store = store;

    for (k, v) in state.accounts.into_iter() {
        if !state.deletes.contains(&k) {
            store.state.set_basic(k, v.basic.balance, v.basic.nonce)?;

            if let Some(code) = v.code {
                store.state.set_code(k, code)?;
            } else {
                store.state.del_code(k)?;
            }
        } else {
            remove(&store, k)?;
        }
    }

    for ((addr, index), value) in state.storages.into_iter() {
        store.state.set_storage(addr, index, value)?;
    }

    Ok(state.logs)
}
