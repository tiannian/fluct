use evm::{
    executor::stack::{StackExecutor, StackSubstateMetadata},
    Config, ExitReason,
};
use fluct_core::{KeyValueStoreReadonly, Store};
use primitive_types::{H160, H256, U256};

use crate::{
    stack::{CoreBackend, CoreStackState, State},
    AddressRecorder, CoreVicinity, Precompiles,
};

pub struct Runtime<'a, KV, R> {
    pub(crate) config: Config,
    pub(crate) vicinity: CoreVicinity,
    pub(crate) recoder: Option<R>,
    pub(crate) store: &'a Store<KV>,
    pub state: Option<State>,
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
        let backend = CoreBackend::new(
            &self.store.block,
            &self.store.state,
            &self.vicinity,
            from,
            self.recoder.as_mut(),
        );
        let state = CoreStackState::new(metadata, &backend);
        let precompiles = Precompiles::default();
        let mut executor = StackExecutor::new_with_precompiles(state, &self.config, &precompiles);
        let res = executor.transact_call(from, to, value, data, gas_limit, Vec::new());

        let state = executor.into_state().deconstruct();
        self.state = Some(state);

        res
    }

    pub fn create(
        &mut self,
        from: H160,
        value: U256,
        init_code: Vec<u8>,
        gas_limit: u64,
    ) -> (ExitReason, Vec<u8>) {
        let metadata = StackSubstateMetadata::new(gas_limit, &self.config);
        let backend = CoreBackend::new(
            &self.store.block,
            &self.store.state,
            &self.vicinity,
            from,
            self.recoder.as_mut(),
        );
        let state = CoreStackState::new(metadata, &backend);
        let precompiles = Precompiles::default();
        let mut executor = StackExecutor::new_with_precompiles(state, &self.config, &precompiles);

        let res = executor.transact_create(from, value, init_code, gas_limit, Vec::new());

        let state = executor.into_state().deconstruct();

        self.state = Some(state);

        res
    }

    pub fn create2(
        &mut self,
        from: H160,
        value: U256,
        init_code: Vec<u8>,
        salt: H256,
        gas_limit: u64,
    ) -> (ExitReason, Vec<u8>) {
        let metadata = StackSubstateMetadata::new(gas_limit, &self.config);
        let backend = CoreBackend::new(
            &self.store.block,
            &self.store.state,
            &self.vicinity,
            from,
            self.recoder.as_mut(),
        );
        let state = CoreStackState::new(metadata, &backend);
        let precompiles = Precompiles::default();
        let mut executor = StackExecutor::new_with_precompiles(state, &self.config, &precompiles);

        let res = executor.transact_create2(from, value, init_code, salt, gas_limit, Vec::new());

        let state = executor.into_state().deconstruct();
        self.state = Some(state);

        res
    }
}
