use std::{collections::BTreeSet, mem};

use evm::{
    backend::{Backend, Basic},
    executor::stack::{Accessed, Log, MemoryStackAccount, StackSubstateMetadata},
    ExitError, Transfer,
};
use primitive_types::{H160, H256, U256};

use super::State;

#[derive(Clone, Debug)]
pub struct CoreStackSubstate<'config> {
    metadata: StackSubstateMetadata<'config>,
    parent: Option<Box<CoreStackSubstate<'config>>>,
    state: State,
}

impl<'config> CoreStackSubstate<'config> {
    pub fn new(metadata: StackSubstateMetadata<'config>, state: Option<State>) -> Self {
        let state = if let Some(s) = state {
            s
        } else {
            State::default()
        };

        Self {
            metadata,
            parent: None,
            state,
        }
    }

    pub fn logs(&self) -> &[Log] {
        &self.state.logs
    }

    pub fn logs_mut(&mut self) -> &mut Vec<Log> {
        &mut self.state.logs
    }

    pub fn metadata(&self) -> &StackSubstateMetadata<'config> {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut StackSubstateMetadata<'config> {
        &mut self.metadata
    }

    /// Deconstruct the executor, return state to be applied. Panic if the
    /// executor is not in the top-level substate.
    #[must_use]
    pub fn deconstruct(self) -> State {
        self.state
    }

    pub fn enter(&mut self, gas_limit: u64, is_static: bool) {
        let mut entering = Self {
            metadata: self.metadata.spit_child(gas_limit, is_static),
            parent: None,
            state: State::default(),
        };
        mem::swap(&mut entering, self);

        self.parent = Some(Box::new(entering));
    }

    pub fn exit_commit(&mut self) -> Result<(), ExitError> {
        let mut exited = *self.parent.take().expect("Cannot commit on root substate");
        mem::swap(&mut exited, self);

        self.metadata.swallow_commit(exited.metadata)?;
        self.state.logs.append(&mut exited.state.logs);

        let mut resets = BTreeSet::new();
        for (address, account) in &exited.state.accounts {
            if account.reset {
                resets.insert(*address);
            }
        }
        let mut reset_keys = BTreeSet::new();
        for (address, key) in self.state.storages.keys() {
            if resets.contains(address) {
                reset_keys.insert((*address, *key));
            }
        }
        for (address, key) in reset_keys {
            self.state.storages.remove(&(address, key));
        }

        self.state.accounts.append(&mut exited.state.accounts);
        self.state.storages.append(&mut exited.state.storages);
        self.state.deletes.append(&mut exited.state.deletes);

        Ok(())
    }

    pub fn exit_revert(&mut self) -> Result<(), ExitError> {
        let mut exited = *self.parent.take().expect("Cannot discard on root substate");
        mem::swap(&mut exited, self);

        self.metadata.swallow_revert(exited.metadata)?;

        Ok(())
    }

    pub fn exit_discard(&mut self) -> Result<(), ExitError> {
        let mut exited = *self.parent.take().expect("Cannot discard on root substate");
        mem::swap(&mut exited, self);

        self.metadata.swallow_discard(exited.metadata)?;

        Ok(())
    }

    pub fn known_account(&self, address: H160) -> Option<&MemoryStackAccount> {
        if let Some(account) = self.state.accounts.get(&address) {
            Some(account)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.known_account(address)
        } else {
            None
        }
    }

    pub fn known_basic(&self, address: H160) -> Option<Basic> {
        self.known_account(address).map(|acc| acc.basic.clone())
    }

    pub fn known_code(&self, address: H160) -> Option<Vec<u8>> {
        self.known_account(address).and_then(|acc| acc.code.clone())
    }

    pub fn known_empty(&self, address: H160) -> Option<bool> {
        if let Some(account) = self.known_account(address) {
            if account.basic.balance != U256::zero() {
                return Some(false);
            }

            if account.basic.nonce != U256::zero() {
                return Some(false);
            }

            if let Some(code) = &account.code {
                return Some(
                    account.basic.balance == U256::zero()
                        && account.basic.nonce == U256::zero()
                        && code.is_empty(),
                );
            }
        }

        None
    }

    pub fn known_storage(&self, address: H160, key: H256) -> Option<H256> {
        if let Some(value) = self.state.storages.get(&(address, key)) {
            return Some(*value);
        }

        if let Some(account) = self.state.accounts.get(&address) {
            if account.reset {
                return Some(H256::default());
            }
        }

        if let Some(parent) = self.parent.as_ref() {
            return parent.known_storage(address, key);
        }

        None
    }

    pub fn known_original_storage(&self, address: H160) -> Option<H256> {
        if let Some(account) = self.state.accounts.get(&address) {
            if account.reset {
                return Some(H256::default());
            }
        }

        if let Some(parent) = self.parent.as_ref() {
            return parent.known_original_storage(address);
        }

        None
    }

    pub fn is_cold(&self, address: H160) -> bool {
        self.recursive_is_cold(&|a| a.accessed_addresses.contains(&address))
    }

    pub fn is_storage_cold(&self, address: H160, key: H256) -> bool {
        self.recursive_is_cold(&|a: &Accessed| a.accessed_storage.contains(&(address, key)))
    }

    fn recursive_is_cold<F: Fn(&Accessed) -> bool>(&self, f: &F) -> bool {
        let local_is_accessed = self.metadata.accessed().as_ref().map(f).unwrap_or(false);
        if local_is_accessed {
            false
        } else {
            self.parent
                .as_ref()
                .map(|p| p.recursive_is_cold(f))
                .unwrap_or(true)
        }
    }

    pub fn deleted(&self, address: H160) -> bool {
        if self.state.deletes.contains(&address) {
            return true;
        }

        if let Some(parent) = self.parent.as_ref() {
            return parent.deleted(address);
        }

        false
    }

    pub fn account_mut<B: Backend>(
        &mut self,
        address: H160,
        backend: &B,
    ) -> &mut MemoryStackAccount {
        if self.state.accounts.get(&address).is_none() {
            let account = self
                .known_account(address)
                .cloned()
                .map(|mut v| {
                    v.reset = false;
                    v
                })
                .unwrap_or_else(|| MemoryStackAccount {
                    basic: backend.basic(address),
                    code: None,
                    reset: false,
                });
            self.state.accounts.insert(address, account);
        }

        self.state
            .accounts
            .get_mut(&address)
            .expect("New account was just inserted")
    }

    pub fn inc_nonce<B: Backend>(&mut self, address: H160, backend: &B) {
        self.account_mut(address, backend).basic.nonce += U256::one();
    }

    pub fn set_storage(&mut self, address: H160, key: H256, value: H256) {
        self.state.storages.insert((address, key), value);
    }

    pub fn reset_storage<B: Backend>(&mut self, address: H160, backend: &B) {
        let mut removing = Vec::new();

        for (oa, ok) in self.state.storages.keys() {
            if *oa == address {
                removing.push(*ok);
            }
        }

        for ok in removing {
            self.state.storages.remove(&(address, ok));
        }

        self.account_mut(address, backend).reset = true;
    }

    pub fn log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) {
        self.state.logs.push(Log {
            address,
            topics,
            data,
        });
    }

    pub fn set_deleted(&mut self, address: H160) {
        self.state.deletes.insert(address);
    }

    pub fn set_code<B: Backend>(&mut self, address: H160, code: Vec<u8>, backend: &B) {
        self.account_mut(address, backend).code = Some(code);
    }

    pub fn transfer<B: Backend>(
        &mut self,
        transfer: Transfer,
        backend: &B,
    ) -> Result<(), ExitError> {
        {
            let source = self.account_mut(transfer.source, backend);
            if source.basic.balance < transfer.value {
                return Err(ExitError::OutOfFund);
            }
            source.basic.balance -= transfer.value;
        }

        {
            let target = self.account_mut(transfer.target, backend);
            target.basic.balance = target.basic.balance.saturating_add(transfer.value);
        }

        Ok(())
    }

    // Only needed for jsontests.
    pub fn withdraw<B: Backend>(
        &mut self,
        address: H160,
        value: U256,
        backend: &B,
    ) -> Result<(), ExitError> {
        let source = self.account_mut(address, backend);
        if source.basic.balance < value {
            return Err(ExitError::OutOfFund);
        }
        source.basic.balance -= value;

        Ok(())
    }

    // Only needed for jsontests.
    pub fn deposit<B: Backend>(&mut self, address: H160, value: U256, backend: &B) {
        let target = self.account_mut(address, backend);
        target.basic.balance = target.basic.balance.saturating_add(value);
    }

    pub fn reset_balance<B: Backend>(&mut self, address: H160, backend: &B) {
        self.account_mut(address, backend).basic.balance = U256::zero();
    }

    pub fn touch<B: Backend>(&mut self, address: H160, backend: &B) {
        self.account_mut(address, backend);
    }
}
